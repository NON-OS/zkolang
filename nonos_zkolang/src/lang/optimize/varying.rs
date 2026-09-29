/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The names a loop body binds, whose values change across the unrolled iterations. */

use alloc::string::String;
use alloc::vec::Vec;

use crate::lang::parse::Stmt;

/*
 * Names bound anywhere inside a loop body. Such a name's value evolves across the loop's
 * unrolled iterations, so its binding is not a constant even when one iteration folds to a
 * literal, and it must never be propagated.
 */
pub(super) fn loop_bound(stmts: &[Stmt], set: &mut Vec<String>, in_loop: bool) {
    for s in stmts {
        match s {
            Stmt::Let(n, _) | Stmt::Input(n) | Stmt::Secret(n) if in_loop => {
                if !set.contains(n) {
                    set.push(n.clone());
                }
            }
            Stmt::LetTuple(names, _) if in_loop => {
                for n in names {
                    if !set.contains(n) {
                        set.push(n.clone());
                    }
                }
            }
            Stmt::For { body, .. } => loop_bound(body, set, true),
            _ => {}
        }
    }
}
