// NONOS Operating System (AGPL-3.0-or-later)

use super::parts::{intent_parts, Spend};
use super::settle::Settle;
use crate::crypto::stark::air::WiredMultiGen;
use crate::crypto::stark::field::Fp;
use crate::shield::batch::assemble;
use crate::shield::key::Break;
use crate::shield::member::TREE_DEPTH;
use crate::shield::note::Note;
use alloc::vec::Vec;

pub struct JoinSplit {
    pub wired: WiredMultiGen,
    pub witness: Vec<Fp>,
    pub intent: Vec<Fp>,
}

pub fn join_split(
    inputs: [Spend; 2],
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    st: Settle,
    flip: Option<usize>,
) -> JoinSplit {
    join_split_at(TREE_DEPTH, inputs, outputs, public_amount, fee, brk, st, flip)
}

/// One intent is a batch of one, so the layout lives in a single place. Depth is
/// open so a forgery can run on a minimal instance.
pub fn join_split_at(
    depth: usize,
    inputs: [Spend; 2],
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    st: Settle,
    flip: Option<usize>,
) -> JoinSplit {
    let p = intent_parts(inputs, outputs, public_amount, fee, brk, st, flip, depth);
    let mut b = assemble(alloc::vec![p]);
    JoinSplit { wired: b.wired, witness: b.witness, intent: b.intents.remove(0) }
}
