/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A random condition of a generated program. */

use crate::flow_gen::{expr, Scope};
use crate::prop_gen::Rng;

/** A random condition. */
pub(crate) fn cond(r: &mut Rng, scope: &Scope) -> String {
    let (x, y) = (expr(r, scope, 1), expr(r, scope, 1));
    let op = ["<", "<=", "==", "!=", ">"][r.below(5) as usize];
    match r.below(4) {
        0 => format!(
            "{x} {op} {y} && {} < {}",
            expr(r, scope, 0),
            expr(r, scope, 0)
        ),
        1 => format!(
            "{x} {op} {y} || {} == {}",
            expr(r, scope, 0),
            expr(r, scope, 0)
        ),
        _ => format!("{x} {op} {y}"),
    }
}
