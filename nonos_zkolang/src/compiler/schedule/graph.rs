/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What scheduling needs to know of each instruction. */

use alloc::vec::Vec;

use super::graph_parts::{edges, is_read, last_sinks, needed, needs};
use crate::compiler::ssa::{Inst, Ssa};

/** The dependences of a program, its sinks, and an estimate of each value's registers. */
pub(super) struct Graph {
    /** The instructions each one needs before it: operands, and a hint's values. */
    pub(super) deps: Vec<Vec<usize>>,
    /** How many of each instruction's dependences, the first, it reads from registers. */
    pub(super) reads: Vec<usize>,
    /** The instructions reading each value from a register. */
    pub(super) users: Vec<Vec<usize>>,
    /** Constraints, outputs, and inverses and selections no register reads, in order. */
    pub(super) sinks: Vec<usize>,
    /** Whether each is a sink, and whether it computes from registers alone. */
    pub(super) sink: Vec<bool>,
    pub(super) op: Vec<bool>,
    /** Whether each value can be brought back after its register is taken. */
    pub(super) recoverable: Vec<bool>,
    /** The last sink that needs each instruction, directly or not. */
    pub(super) last: Vec<usize>,
    /** How many registers evaluating each value takes, as for a tree. */
    pub(super) need: Vec<u32>,
}

impl Graph {
    pub(super) fn of(ssa: &Ssa) -> Graph {
        let n = ssa.insts.len();
        let (deps, reads, users) = edges(ssa);
        let op: Vec<bool> = ssa
            .insts
            .iter()
            .map(|i| !i.is_effect() && !is_read(i))
            .collect();
        let sink: Vec<bool> = (0..n)
            .map(|i| {
                ssa.insts[i].is_effect() || op[i] && users[i].is_empty() && needed(&ssa.insts[i])
            })
            .collect();
        let sinks: Vec<usize> = (0..n).filter(|&i| sink[i]).collect();
        let recoverable = ssa
            .insts
            .iter()
            .map(|i| recoverable(i, ssa.n_public))
            .collect();
        let last = last_sinks(&deps, &sinks);
        let need = needs(&deps, &op);
        Graph {
            deps,
            reads,
            users,
            sinks,
            sink,
            op,
            recoverable,
            last,
            need,
        }
    }
}

/** Whether `i` can be brought back after its register is taken: a constant, a public input. */
fn recoverable(i: &Inst, n_public: u16) -> bool {
    matches!(i, Inst::Const(_)) || matches!(i, Inst::Input(k) if *k < n_public)
}
