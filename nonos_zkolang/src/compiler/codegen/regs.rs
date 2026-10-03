/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Registers: taking a free one, freeing one when none is. Only a value that can be brought
 * back is freed while it is still needed: a constant, written again, or a public input,
 * whose every read the proof pins to the committed value. Of those, the one needed
 * furthest ahead goes.
 */

use super::machine::CodegenError;
use super::state::Cg;
use crate::compiler::ssa::{Inst, V};
use crate::isa::REGS;

impl<'a> Cg<'a> {
    /** Whether `v` can be brought back into a register after its register is taken. */
    pub(super) fn recoverable(&self, v: V) -> bool {
        match self.ssa.get(v) {
            Some(Inst::Const(_)) => true,
            Some(Inst::Input(k)) => *k < self.ssa.n_public,
            _ => false,
        }
    }

    /** A free register, freeing one if none is; never one that holds a value of `keep`. */
    pub(super) fn take_reg(&mut self, keep: &[V]) -> Result<u8, CodegenError> {
        let regs = 0..u8::try_from(REGS).unwrap_or(u8::MAX);
        if let Some(r) = regs
            .clone()
            .find(|&r| self.holder[usize::from(r)].is_none())
        {
            return Ok(r);
        }
        let mut best: Option<(u32, u8)> = None;
        for r in regs {
            let Some(v) = self.holder[usize::from(r)] else {
                continue;
            };
            if keep.contains(&v) || !self.recoverable(v) {
                continue;
            }
            let next = self.next_use(v).unwrap_or(u32::MAX);
            if best.is_none_or(|(n, _)| next > n) {
                best = Some((next, r));
            }
        }
        let (_, r) = best.ok_or(CodegenError::Pressure(self.at))?;
        if let Some(v) = self.holder[usize::from(r)] {
            self.release(v);
        }
        Ok(r)
    }

    /** Record that register `r` holds `v`. */
    pub(super) fn bind(&mut self, v: V, r: u8) {
        self.holder[usize::from(r)] = Some(v);
        if let Some(slot) = self.reg.get_mut(v.index()) {
            *slot = Some(r);
        }
    }

    /** Free the register holding `v`, if one does. */
    pub(super) fn release(&mut self, v: V) {
        if let Some(r) = self.reg.get_mut(v.index()).and_then(|r| r.take()) {
            self.holder[usize::from(r)] = None;
        }
    }
}
