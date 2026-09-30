/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Integer powers, checked like every integer operation. */

use super::{FailKind, Value};
use crate::compiler::syntax::IntTy;

/** `a.pow(k)` for an integer of type `t`, checked. */
pub(super) fn int_pow(a: i128, k: i128, t: IntTy) -> Result<Value, FailKind> {
    let mut acc: i128 = 1;
    /* Beyond |a| = 1, each factor at least doubles the magnitude, so 65 steps settle it. */
    let steps = if a.abs() <= 1 {
        k.rem_euclid(2) + 2
    } else {
        k.min(65)
    };
    for _ in 0..steps.min(k) {
        acc = acc
            .checked_mul(a)
            .filter(|v| t.contains(*v))
            .ok_or(FailKind::Overflow)?;
    }
    Ok(Value::Int(acc))
}
