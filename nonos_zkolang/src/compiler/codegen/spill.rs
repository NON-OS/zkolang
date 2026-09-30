/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Spilling: a value no register can keep, which cannot be read again, is copied into the
 * advice. The copy is read into the scratch register, the value subtracted and the
 * difference asserted zero, so the copy equals the value and can be read back later.
 */

use super::machine::{CodegenError, Origin};
use super::state::{Cg, SCRATCH};
use crate::compiler::ssa::{Hint, V};
use crate::isa::Op;

impl<'a> Cg<'a> {
    /** Copy `v`, in register `r`, into the advice, constrained equal to it. */
    pub(super) fn spill(&mut self, v: V, r: u8) -> Result<(), CodegenError> {
        let s = self.new_copy(v).ok_or(CodegenError::TooManySlots)?;
        if let Some(slot) = self.slot.get_mut(v.index()) {
            *slot = Some(s);
        }
        let idx = self.index(s)?;
        let (d, o) = (SCRATCH, Origin::Spill(v));
        self.push(Op::Inp { d, idx }, o);
        self.push(Op::Sub { d, a: d, b: r }, o);
        self.push(Op::Assert { a: d }, o);
        Ok(())
    }

    /** The input index of advice slot `s`. */
    pub(super) fn index(&self, s: u16) -> Result<u16, CodegenError> {
        let i = self.out.n_inputs.checked_add(usize::from(s));
        i.and_then(|i| u16::try_from(i).ok())
            .ok_or(CodegenError::TooManySlots)
    }

    /** Append an instruction and where it comes from. */
    pub(super) fn push(&mut self, op: Op, o: Origin) {
        self.out.ops.push(op);
        self.out.origins.push(o);
    }

    /** Add a spill copy of `v` to the advice and return its slot. */
    pub(super) fn new_copy(&mut self, v: V) -> Option<u16> {
        let s = u16::try_from(self.out.advice.len()).ok()?;
        self.out.advice.push(Hint::Copy(v));
        Some(s)
    }
}
