/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The state of the allocation walk. */

use alloc::vec;
use alloc::vec::Vec;

use super::machine::Machine;
use crate::compiler::ssa::{Inst, Ssa, V};
use crate::isa::REGS;

/** The register kept free for checking spilled copies. */
pub(super) const SCRATCH: u8 = (REGS - 1) as u8;

/** The walk: the program so far, what each register holds, and where each value is. */
pub(super) struct Cg<'a> {
    pub(super) ssa: &'a Ssa,
    pub(super) out: Machine,
    /** The register holding each value, if one does. */
    pub(super) reg: Vec<Option<u8>>,
    /** The value each register holds. */
    pub(super) holder: [Option<V>; REGS],
    /** Each value's advice slot: its own for an advice value, else its copy's, if any. */
    pub(super) slot: Vec<Option<u16>>,
    /** The positions of each value's register reads, and how many have passed. */
    pub(super) uses: Vec<Vec<u32>>,
    pub(super) passed: Vec<usize>,
}

impl<'a> Cg<'a> {
    /** The start of a walk over `ssa`, each advice value given its slot. */
    pub(super) fn new(ssa: &'a Ssa) -> Cg<'a> {
        let n = ssa.insts.len();
        let mut uses = vec![Vec::new(); n];
        let (mut slot, mut advice) = (vec![None; n], Vec::new());
        for (i, inst) in ssa.insts.iter().enumerate() {
            let at = u32::try_from(i).unwrap_or(u32::MAX);
            for v in inst.operands() {
                if let Some(u) = uses.get_mut(v.index()) {
                    u.push(at);
                }
            }
            if let Inst::Advice(h) = inst {
                slot[i] = u16::try_from(advice.len()).ok();
                advice.push(*h);
            }
        }
        let out = Machine {
            advice,
            n_inputs: ssa.n_inputs(),
            ..Machine::default()
        };
        Cg {
            ssa,
            out,
            reg: vec![None; n],
            holder: [None; REGS],
            slot,
            uses,
            passed: vec![0; n],
        }
    }

    /** The next position `v` is read at, if any. */
    pub(super) fn next_use(&self, v: V) -> Option<u32> {
        let (u, p) = (self.uses.get(v.index())?, *self.passed.get(v.index())?);
        u.get(p).copied()
    }
}
