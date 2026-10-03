/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Replaying one machine instruction against the SSA value it claims. */

use alloc::vec::Vec;

use super::machine::Machine;
use crate::compiler::ssa::{Inst, Ssa, V};
use crate::isa::{Op, REGS};

/** The replay: what each register holds, which values are done, which slots were read. */
pub(super) struct Replay<'a> {
    pub(super) ssa: &'a Ssa,
    pub(super) m: &'a Machine,
    pub(super) regs: [Option<V>; REGS],
    pub(super) done: Vec<bool>,
    /** Whether each input and advice slot has been read. */
    pub(super) read: Vec<bool>,
    /** Each advice value's slot: how many advice values precede it. */
    pub(super) own: Vec<Option<usize>>,
}

impl<'a> Replay<'a> {
    pub(super) fn set(&mut self, r: u8, v: Option<V>) {
        if let Some(slot) = self.regs.get_mut(usize::from(r)) {
            *slot = v;
        }
    }

    /** The advice slot an input index names. */
    pub(super) fn slot(&self, idx: u16) -> Option<usize> {
        usize::from(idx).checked_sub(self.m.n_inputs)
    }

    /** The advice slot of the advice value `v`. */
    fn own_slot(&self, v: V) -> Option<usize> {
        self.own.get(v.index()).copied().flatten()
    }

    /** Whether `op` reads `v` into a register, and which. */
    pub(super) fn reads(&self, op: Op, v: V) -> Option<u8> {
        match (self.ssa.get(v)?, op) {
            (Inst::Const(c), Op::Imm { d, v: f }) if f.value() == *c => Some(d),
            (Inst::Input(k), Op::Inp { d, idx }) if idx == *k => Some(d),
            (Inst::Advice(_), Op::Inp { d, idx }) => {
                let own = self.own_slot(v)?;
                (self.slot(idx) == Some(own)).then_some(d)
            }
            _ => None,
        }
    }
}
