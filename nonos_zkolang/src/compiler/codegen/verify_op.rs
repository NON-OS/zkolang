/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Replaying one machine instruction against the SSA value it claims. */

use alloc::vec::Vec;

use super::machine::Machine;
use crate::compiler::ssa::{Hint, Inst, Ssa, V};
use crate::isa::{Op, REGS};

/** The replay: what each register holds, which values are done, which copies hold. */
pub(super) struct Replay<'a> {
    pub(super) ssa: &'a Ssa,
    pub(super) m: &'a Machine,
    pub(super) regs: [Option<V>; REGS],
    pub(super) done: Vec<bool>,
    pub(super) copies: Vec<bool>,
    /** A spill being checked: its value, slot, register and how far it got. */
    pub(super) spill: Option<(V, usize, u8, u8)>,
}

impl<'a> Replay<'a> {
    pub(super) fn holds(&self, r: u8, v: V) -> bool {
        self.regs.get(usize::from(r)).copied().flatten() == Some(v)
    }

    pub(super) fn set(&mut self, r: u8, v: Option<V>) {
        if let Some(slot) = self.regs.get_mut(usize::from(r)) {
            *slot = v;
        }
    }

    /** The advice slot an input index names. */
    pub(super) fn slot(&self, idx: u16) -> Option<usize> {
        usize::from(idx).checked_sub(self.m.n_inputs)
    }

    /** The advice slot of the advice value `v`: how many advice values precede it. */
    fn own_slot(&self, v: V) -> Option<usize> {
        let before = self.ssa.insts.get(..v.index())?;
        Some(
            before
                .iter()
                .filter(|i| matches!(i, Inst::Advice(_)))
                .count(),
        )
    }

    /** Whether `op` reads `v` again into a register, and which. */
    fn reads_again(&self, op: Op, v: V) -> Option<u8> {
        match (self.ssa.get(v)?, op) {
            (Inst::Const(c), Op::Imm { d, v: f }) if f.value() == *c => Some(d),
            (Inst::Input(k), Op::Inp { d, idx }) if idx == *k => Some(d),
            (Inst::Advice(_), Op::Inp { d, idx }) if self.slot(idx) == self.own_slot(v) => Some(d),
            (_, Op::Inp { d, idx }) => {
                let s = self.slot(idx)?;
                let copy = self.m.advice.get(s) == Some(&Hint::Copy(v));
                (copy && self.copies.get(s).copied().unwrap_or(false)).then_some(d)
            }
            _ => None,
        }
    }

    /** A reload of `v`. */
    pub(super) fn reload(&mut self, op: Op, v: V) -> Result<(), &'static str> {
        let d = self
            .reads_again(op, v)
            .ok_or("a reload that does not read the value")?;
        self.set(d, Some(v));
        Ok(())
    }
}
