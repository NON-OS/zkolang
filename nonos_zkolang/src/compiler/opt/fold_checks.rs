/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Folding constraints and gadgets on constants: one that holds is dropped or valued. */

use nonos_stark::field::Fp;

use super::fold_rules::Folded;
use crate::compiler::ssa::{Inst, V};

/** How the constraint or gadget `inst` folds, `k` giving each constant operand. */
pub(super) fn rule(inst: Inst, k: &dyn Fn(V) -> Option<Fp>) -> Folded {
    let c = |v: V| k(v).map(|x| x.value());
    let below = |x: u64, n: u8| n < 64 && x >> n == 0;
    match inst {
        Inst::AssertBool(a) if c(a).is_some_and(|x| x <= 1) => Folded::Drop,
        Inst::AssertZero(a) if c(a) == Some(0) => Folded::Drop,
        Inst::RangeCheck(a, n) if c(a).is_some_and(|x| below(x, n)) => Folded::Drop,
        Inst::Bit(a, i, n) => match c(a) {
            Some(x) if below(x, n) => Folded::Const(x.checked_shr(u32::from(i)).unwrap_or(0) & 1),
            _ => Folded::Keep,
        },
        Inst::FieldBit(a, i) => match c(a) {
            Some(x) => Folded::Const(x.checked_shr(u32::from(i)).unwrap_or(0) & 1),
            None => Folded::Keep,
        },
        Inst::Quot(a, b, n) | Inst::Rem(a, b, n) => match (c(a), c(b)) {
            (Some(x), Some(y)) if n <= 32 && below(x, n) && below(y, n) && y != 0 => {
                Folded::Const(if matches!(inst, Inst::Quot(..)) {
                    x / y
                } else {
                    x % y
                })
            }
            _ => Folded::Keep,
        },
        _ => Folded::Keep,
    }
}
