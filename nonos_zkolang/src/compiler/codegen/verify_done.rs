/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The end of a replay: every instruction that must appear did; and the slots of the advice. */

use super::verify::VerifyError;
use crate::compiler::ssa::{Inst, Ssa};

/** An error unless every constraint, output, inverse and selection of `ssa` is `done`. */
pub(super) fn missing(ssa: &Ssa, done: &[bool], end: usize) -> Result<(), VerifyError> {
    for (i, inst) in ssa.insts.iter().enumerate() {
        let needed = inst.is_effect() || matches!(inst, Inst::Inv(_) | Inst::Sel(..));
        if needed && !done.get(i).copied().unwrap_or(false) {
            return Err(VerifyError {
                op: end,
                why: "a constraint, output, inverse or selection is missing",
            });
        }
    }
    Ok(())
}

/** Each advice value's slot in `ssa`: how many advice values precede it. */
pub(super) fn own_slots(ssa: &Ssa) -> alloc::vec::Vec<Option<usize>> {
    let mut n = 0;
    ssa.insts
        .iter()
        .map(|i| {
            let s = matches!(i, Inst::Advice(_)).then_some(n);
            n += usize::from(s.is_some());
            s
        })
        .collect()
}
