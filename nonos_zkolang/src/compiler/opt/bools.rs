/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Which values are 0 or 1 in every accepted run: comparisons, bits, constants 0 and 1,
 * products and selections of such values, and any value an `AssertBool` constrains,
 * wherever it stands, since a run in which it fails is not accepted.
 */

use alloc::vec;
use alloc::vec::Vec;

use crate::compiler::ssa::{Inst, Ssa};

/** For each value, whether it is boolean in every accepted run. */
pub(super) fn booleans(ssa: &Ssa) -> Vec<bool> {
    let mut out = vec![false; ssa.insts.len()];
    for inst in &ssa.insts {
        if let Inst::AssertBool(a) = inst {
            if let Some(b) = out.get_mut(a.index()) {
                *b = true;
            }
        }
    }
    for (i, inst) in ssa.insts.iter().enumerate() {
        let is = |v: crate::compiler::ssa::V| out.get(v.index()).copied().unwrap_or(false);
        let known = match *inst {
            Inst::Const(c) => c <= 1,
            Inst::Eq(..) | Inst::Bit(..) | Inst::FieldBit(..) => true,
            Inst::Mul(a, b) | Inst::Sel(_, a, b) => is(a) && is(b),
            _ => false,
        };
        if let Some(b) = out.get_mut(i) {
            *b |= known;
        }
    }
    out
}
