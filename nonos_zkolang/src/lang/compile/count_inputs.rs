/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Count a statement list's public inputs.

use crate::lang::parse::Stmt;

/// The number of public inputs a statement list produces, counting the inputs a loop
/// unrolls to, so the count matches the compiled program even through loops. In
/// practice this stays small, since each input binds a register.
pub(super) fn count_inputs(stmts: &[Stmt]) -> u64 {
    let mut n = 0u64;
    for s in stmts {
        match s {
            Stmt::Input(_) => n += 1,
            Stmt::For { lo, hi, body, .. } => {
                let iters = hi.saturating_sub(*lo);
                n = n.saturating_add(iters.saturating_mul(count_inputs(body)));
            }
            _ => {}
        }
    }
    n
}
