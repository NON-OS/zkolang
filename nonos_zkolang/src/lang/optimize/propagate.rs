/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Constant propagation, the safe form of rematerialization. A binding whose value folds
//! to a constant is inlined at each use and its statement dropped, so the constant is
//! recomputed as an immediate rather than held in a register. This lowers register
//! pressure, which raises the effective ceiling: a program bounded by constant bindings
//! that would exhaust the register file now compiles. A binding to a runtime value stays,
//! and a rebinding shadows, so the transform preserves meaning.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::expr::fold;
use crate::lang::parse::{Expr, Stmt};

type Env = Vec<(String, Option<u64>)>;

fn latest(env: &Env, name: &str) -> Option<u64> {
    env.iter()
        .rev()
        .find(|(n, _)| n == name)
        .and_then(|(_, v)| *v)
}

fn bx(e: Expr) -> Box<Expr> {
    Box::new(e)
}

// Replace each variable bound to a known constant with that constant, recursively.
fn subst(e: &Expr, env: &Env) -> Expr {
    match e {
        Expr::Num(v) => Expr::Num(*v),
        Expr::Var(n) => match latest(env, n) {
            Some(v) => Expr::Num(v),
            None => Expr::Var(n.clone()),
        },
        Expr::Add(a, b) => Expr::Add(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Sub(a, b) => Expr::Sub(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Mul(a, b) => Expr::Mul(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Div(a, b) => Expr::Div(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Neg(x) => Expr::Neg(bx(subst(x, env))),
        Expr::Eq(a, b) => Expr::Eq(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Ne(a, b) => Expr::Ne(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Lt(a, b) => Expr::Lt(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Inv(x) => Expr::Inv(bx(subst(x, env))),
        Expr::Sel(c, a, b) => Expr::Sel(bx(subst(c, env)), bx(subst(a, env)), bx(subst(b, env))),
        Expr::If(c, a, b) => Expr::If(bx(subst(c, env)), bx(subst(a, env)), bx(subst(b, env))),
        Expr::Call(n, args) => Expr::Call(n.clone(), args.iter().map(|a| subst(a, env)).collect()),
        Expr::Index(base, idx, at) => Expr::Index(bx(subst(base, env)), bx(subst(idx, env)), *at),
        Expr::Array(xs) => Expr::Array(xs.iter().map(|a| subst(a, env)).collect()),
    }
}

fn norm(e: &Expr, env: &Env) -> Expr {
    fold(&subst(e, env))
}

// Names bound anywhere inside a loop body. Such a name's value evolves across the loop's
// unrolled iterations, so its binding is not a constant even when one iteration folds to a
// literal, and it must never be propagated.
fn loop_bound(stmts: &[Stmt], set: &mut Vec<String>, in_loop: bool) {
    for s in stmts {
        match s {
            Stmt::Let(n, _) | Stmt::Input(n) | Stmt::Secret(n) if in_loop => {
                if !set.contains(n) {
                    set.push(n.clone());
                }
            }
            Stmt::For { body, .. } => loop_bound(body, set, true),
            _ => {}
        }
    }
}

/// Propagate constants across a statement list.
pub(super) fn propagate(stmts: &[Stmt]) -> Vec<Stmt> {
    let mut env: Env = Vec::new();
    let mut varying: Vec<String> = Vec::new();
    loop_bound(stmts, &mut varying, false);
    go(stmts, &mut env, 0, &varying)
}

fn go(stmts: &[Stmt], env: &mut Env, depth: usize, varying: &[String]) -> Vec<Stmt> {
    let mut out = Vec::new();
    for s in stmts {
        match s {
            Stmt::Let(name, e) => {
                let e2 = norm(e, env);
                let is_varying = varying.iter().any(|n| n == name);
                if let (Expr::Num(v), false) = (&e2, is_varying) {
                    env.push((name.clone(), Some(*v)));
                    // At the top level the binding is dead once its uses are inlined, so
                    // drop it and free its register; inside a loop keep it, since a name
                    // it defines may be read after the loop.
                    if depth != 0 {
                        out.push(Stmt::Let(name.clone(), e2));
                    }
                } else {
                    env.push((name.clone(), None));
                    out.push(Stmt::Let(name.clone(), e2));
                }
            }
            Stmt::Input(n) => {
                env.push((n.clone(), None));
                out.push(Stmt::Input(n.clone()));
            }
            Stmt::Secret(n) => {
                env.push((n.clone(), None));
                out.push(Stmt::Secret(n.clone()));
            }
            Stmt::Output(e) => out.push(Stmt::Output(norm(e, env))),
            Stmt::Assert(e) => out.push(Stmt::Assert(norm(e, env))),
            Stmt::For { var, lo, hi, body } => {
                let mark = env.len();
                env.push((var.clone(), None));
                let nbody = go(body, env, depth + 1, varying);
                env.truncate(mark);
                out.push(Stmt::For {
                    var: var.clone(),
                    lo: *lo,
                    hi: *hi,
                    body: nbody,
                });
            }
        }
    }
    out
}
