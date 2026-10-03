/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The simplifying passes run together until the program stops changing. */

use super::{cse, dce, fold};
use crate::compiler::ssa::Ssa;

/** Fold, number and prune until nothing changes, at most four rounds. */
pub fn simplify(ssa: &Ssa) -> Ssa {
    let mut cur = ssa.clone();
    for _ in 0..4 {
        let next = dce(&cse(&fold(&cur)));
        if next == cur {
            break;
        }
        cur = next;
    }
    cur
}
