/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Count a statement list's secret inputs.

use crate::lang::parse::Stmt;

/// The number of secret inputs a statement list produces, counting a loop's unrolled
/// copies, so the count matches the compiled program. The comparison advice indexes
/// after this total, so a program's advice never collides with its declared secrets.
pub(super) fn count_secrets(stmts: &[Stmt]) -> u64 {
    let mut n = 0u64;
    for s in stmts {
        match s {
            Stmt::Secret(_) => n += 1,
            Stmt::For { lo, hi, body, .. } => {
                let iters = hi.saturating_sub(*lo);
                n = n.saturating_add(iters.saturating_mul(count_secrets(body)));
            }
            _ => {}
        }
    }
    n
}
