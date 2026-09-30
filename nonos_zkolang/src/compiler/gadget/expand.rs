/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The two expansions: division, then range checks and bits. */

use super::rewrite::rebuild;
use crate::compiler::ssa::{Inst, Ssa};

/** Expand division into advice, range checks and machine instructions. */
pub fn expand_division(ssa: &Ssa) -> Ssa {
    rebuild(ssa, |r, inst| match inst {
        Inst::Quot(a, b, n) => r.divide(a, b, n).0,
        Inst::Rem(a, b, n) => r.divide(a, b, n).1,
        _ => r.b.emit(inst),
    })
}

/** Expand every range check and bit into a bit decomposition. */
pub fn expand_bits(ssa: &Ssa) -> Ssa {
    rebuild(ssa, |r, inst| match inst {
        Inst::RangeCheck(v, n) => {
            r.range_check(v, n);
            v
        }
        Inst::Bit(v, k, n) => {
            let bit = r.decompose(v, n).get(usize::from(k)).copied();
            bit.unwrap_or_else(|| r.b.konst(0))
        }
        Inst::FieldBit(v, k) => {
            let bit = r.field_bits(v).get(usize::from(k)).copied();
            bit.unwrap_or_else(|| r.b.konst(0))
        }
        _ => r.b.emit(inst),
    })
}
