/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Propagate through one `let`. */

use alloc::string::String;
use alloc::vec::Vec;

use super::env::Env;
use super::subst::norm;
use crate::lang::parse::{Expr, Stmt};

/**
 * Normalise a `let` and record what its name holds. A binding that folds to a constant and
 * is not rebound inside any loop is recorded as that constant; at the top level it is then
 * dead once its uses are inlined, so it is dropped and its register never taken, while
 * inside a loop it is kept, since a name it defines may be read after the loop.
 */
pub(super) fn let_binding(
    name: &str,
    e: &Expr,
    env: &mut Env,
    in_loop: bool,
    varying: &[String],
    out: &mut Vec<Stmt>,
) {
    let e2 = norm(e, env);
    let is_varying = varying.iter().any(|n| n == name);
    if let (Expr::Num(v), false) = (&e2, is_varying) {
        env.push((String::from(name), Some(*v)));
        if in_loop {
            out.push(Stmt::Let(String::from(name), e2));
        }
    } else {
        env.push((String::from(name), None));
        out.push(Stmt::Let(String::from(name), e2));
    }
}
