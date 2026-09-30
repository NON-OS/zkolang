/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Registers: taking a free one, freeing one when none is. The value freed is one that
 * can be read again, a constant, an input or advice, if any can, else the one read
 * furthest ahead; a value that cannot be read again is first copied into the advice.
 */

use super::machine::CodegenError;
use super::state::{Cg, SCRATCH};
use crate::compiler::ssa::{Inst, V};

impl<'a> Cg<'a> {
    /** Whether `v` can be read again: a constant, an input, or in the advice. */
    pub(super) fn rereadable(&self, v: V) -> bool {
        matches!(self.ssa.get(v), Some(Inst::Const(_) | Inst::Input(_)))
            || self.slot.get(v.index()).is_some_and(|s| s.is_some())
    }

    /** A free register, freeing one if none is; never one that holds a value of `keep`. */
    pub(super) fn take_reg(&mut self, keep: &[V]) -> Result<u8, CodegenError> {
        if let Some(r) = (0..SCRATCH).find(|&r| self.holder[usize::from(r)].is_none()) {
            return Ok(r);
        }
        let mut best: Option<((bool, u32), u8)> = None;
        for r in 0..SCRATCH {
            let Some(v) = self.holder[usize::from(r)] else {
                continue;
            };
            if keep.contains(&v) {
                continue;
            }
            let key = (self.rereadable(v), self.next_use(v).unwrap_or(u32::MAX));
            if best.is_none_or(|(k, _)| key > k) {
                best = Some((key, r));
            }
        }
        let (_, r) = best.ok_or(CodegenError::Lost(V(u32::MAX)))?;
        if let Some(v) = self.holder[usize::from(r)] {
            if !self.rereadable(v) && self.next_use(v).is_some() {
                self.spill(v, r)?;
            }
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
