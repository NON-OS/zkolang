/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Common-subexpression elimination. A pure subexpression that appears more than once in a
//! statement is computed once into a fresh binding and its occurrences replaced, so the
//! trace holds one copy instead of several. Only pure arithmetic subtrees are shared, so
//! nothing that carries a constraint (an inverse, a division, a comparison, a select) is
//! moved or merged, and the transform cannot change what a program proves. The fresh names
//! use a character a source identifier cannot, so they never collide with a program's own.

use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::lang::parse::{Expr, Stmt};

fn is_pure(e: &Expr) -> bool {
    match e {
        Expr::Num(_) | Expr::Var(_) => true,
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Eq(a, b) | Expr::Ne(a, b) => {
            is_pure(a) && is_pure(b)
        }
        Expr::Neg(x) => is_pure(x),
        _ => false,
    }
}

fn hoistable(e: &Expr) -> bool {
    is_pure(e) && !matches!(e, Expr::Num(_) | Expr::Var(_))
}

fn children(e: &Expr) -> Vec<&Expr> {
    match e {
        // An index must fold to a compile-time constant, so nothing inside one may be
        // hoisted into a binding; treat the whole index expression as opaque.
        Expr::Num(_) | Expr::Var(_) | Expr::Index(_, _) => Vec::new(),
        Expr::Neg(x) | Expr::Inv(x) => vec![x.as_ref()],
        Expr::Add(a, b)
        | Expr::Sub(a, b)
        | Expr::Mul(a, b)
        | Expr::Div(a, b)
        | Expr::Eq(a, b)
        | Expr::Ne(a, b)
        | Expr::Lt(a, b) => vec![a.as_ref(), b.as_ref()],
        Expr::Sel(c, a, b) | Expr::If(c, a, b) => vec![c.as_ref(), a.as_ref(), b.as_ref()],
        Expr::Call(_, args) | Expr::Array(args) => args.iter().collect(),
    }
}

fn size(e: &Expr) -> usize {
    1 + children(e).iter().map(|c| size(c)).sum::<usize>()
}

fn count(h: &Expr, n: &Expr) -> usize {
    if h == n {
        1
    } else {
        children(h).iter().map(|c| count(c, n)).sum()
    }
}

fn collect(e: &Expr, out: &mut Vec<Expr>) {
    if hoistable(e) {
        out.push(e.clone());
    }
    for c in children(e) {
        collect(c, out);
    }
}

fn bx(e: Expr) -> Box<Expr> {
    Box::new(e)
}

fn replace(e: &Expr, n: &Expr, name: &str) -> Expr {
    if e == n {
        return Expr::Var(String::from(name));
    }
    match e {
        Expr::Num(v) => Expr::Num(*v),
        Expr::Var(v) => Expr::Var(v.clone()),
        Expr::Add(a, b) => Expr::Add(bx(replace(a, n, name)), bx(replace(b, n, name))),
        Expr::Sub(a, b) => Expr::Sub(bx(replace(a, n, name)), bx(replace(b, n, name))),
        Expr::Mul(a, b) => Expr::Mul(bx(replace(a, n, name)), bx(replace(b, n, name))),
        Expr::Div(a, b) => Expr::Div(bx(replace(a, n, name)), bx(replace(b, n, name))),
        Expr::Neg(x) => Expr::Neg(bx(replace(x, n, name))),
        Expr::Eq(a, b) => Expr::Eq(bx(replace(a, n, name)), bx(replace(b, n, name))),
        Expr::Ne(a, b) => Expr::Ne(bx(replace(a, n, name)), bx(replace(b, n, name))),
        Expr::Lt(a, b) => Expr::Lt(bx(replace(a, n, name)), bx(replace(b, n, name))),
        Expr::Inv(x) => Expr::Inv(bx(replace(x, n, name))),
        Expr::Sel(c, a, b) => Expr::Sel(
            bx(replace(c, n, name)),
            bx(replace(a, n, name)),
            bx(replace(b, n, name)),
        ),
        Expr::If(c, a, b) => Expr::If(
            bx(replace(c, n, name)),
            bx(replace(a, n, name)),
            bx(replace(b, n, name)),
        ),
        Expr::Call(f, args) => Expr::Call(
            f.clone(),
            args.iter().map(|a| replace(a, n, name)).collect(),
        ),
        // Left opaque so an index expression is never rewritten and stays constant.
        Expr::Index(a, b) => Expr::Index(a.clone(), b.clone()),
        Expr::Array(xs) => Expr::Array(xs.iter().map(|a| replace(a, n, name)).collect()),
    }
}

// Hoist repeated pure subexpressions of `e` into let statements appended to `pre`, largest
// first, returning the rewritten expression. Terminates because each hoist replaces two or
// more occurrences with one variable, which is not itself hoistable.
fn hoist(mut e: Expr, pre: &mut Vec<Stmt>, ctr: &mut usize) -> Expr {
    loop {
        let mut cands = Vec::new();
        collect(&e, &mut cands);
        let mut best: Option<Expr> = None;
        let mut best_size = 1;
        for c in &cands {
            if size(c) > best_size && count(&e, c) >= 2 {
                best_size = size(c);
                best = Some(c.clone());
            }
        }
        match best {
            Some(s) => {
                let name = format!("$cse{}", *ctr);
                *ctr += 1;
                pre.push(Stmt::Let(name.clone(), s.clone()));
                e = replace(&e, &s, &name);
            }
            None => return e,
        }
    }
}

/// Share repeated pure subexpressions across a statement list.
pub(super) fn cse(stmts: &[Stmt]) -> Vec<Stmt> {
    let mut ctr = 0usize;
    go(stmts, &mut ctr)
}

fn go(stmts: &[Stmt], ctr: &mut usize) -> Vec<Stmt> {
    let mut out = Vec::new();
    for s in stmts {
        match s {
            Stmt::Let(n, e) => {
                let e2 = hoist(e.clone(), &mut out, ctr);
                out.push(Stmt::Let(n.clone(), e2));
            }
            Stmt::Output(e) => {
                let e2 = hoist(e.clone(), &mut out, ctr);
                out.push(Stmt::Output(e2));
            }
            Stmt::Assert(e) => {
                let e2 = hoist(e.clone(), &mut out, ctr);
                out.push(Stmt::Assert(e2));
            }
            Stmt::Input(n) => out.push(Stmt::Input(n.clone())),
            Stmt::Secret(n) => out.push(Stmt::Secret(n.clone())),
            Stmt::For { var, lo, hi, body } => out.push(Stmt::For {
                var: var.clone(),
                lo: *lo,
                hi: *hi,
                body: go(body, ctr),
            }),
        }
    }
    out
}
