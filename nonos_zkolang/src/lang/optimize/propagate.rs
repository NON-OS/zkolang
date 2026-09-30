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

use alloc::string::String;
use alloc::vec::Vec;

use super::assert_form::norm_assert;
use super::env::Env;
use super::let_binding::let_binding;
use super::subst::norm;
use super::varying::loop_bound;
use crate::lang::parse::Stmt;

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
            Stmt::Let(name, e) => let_binding(name, e, env, depth != 0, varying, &mut out),
            Stmt::LetTuple(names, e) => {
                /*
                 * The destructured values are runtime, so their names are not constants. A
                 * `_` slot binds nothing, as in lowering, and hides no earlier `_`.
                 */
                let e2 = norm(e, env);
                for n in names.iter().filter(|n| *n != "_") {
                    env.push((n.clone(), None));
                }
                out.push(Stmt::LetTuple(names.clone(), e2));
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
            Stmt::Assert(e) => out.extend(norm_assert(e, env)),
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
