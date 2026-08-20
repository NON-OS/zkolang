// NONOS Operating System (AGPL-3.0-or-later)

use super::parts::{intent_parts, Spend};
use super::settle::Settle;
use super::witness::Places;
use crate::crypto::stark::air::WiredMultiExt;
use crate::crypto::stark::field::Fp;
use crate::shield::batch::assemble;
use crate::shield::key::Break;
use crate::shield::member::TREE_DEPTH;
use crate::shield::note::Note;
use alloc::vec::Vec;

pub(crate) struct JoinSplit {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
    pub intent: Vec<Fp>,
}

pub(crate) fn join_split(
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
pub(crate) fn join_split_at(
    depth: usize,
    inputs: [Spend; 2],
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    st: Settle,
    flip: Option<usize>,
) -> JoinSplit {
    let p = intent_parts(inputs, outputs, public_amount, fee, brk, st, flip, depth, None);
    let mut b = assemble(alloc::vec![p]);
    JoinSplit { wired: b.wired, witness: b.witness, intent: b.intents.remove(0) }
}

/// A spend against the pool the contract owns: the caller reads the paths and the
/// published root out of it. The circuit never builds a tree, which is the whole
/// difference between proving membership and proving you can build a tree holding
/// your own note.
pub(crate) fn join_split_placed(
    inputs: [Spend; 2],
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    st: Settle,
    at: &Places,
) -> JoinSplit {
    let depth = at.note[0].depth();
    let p =
        intent_parts(inputs, outputs, public_amount, fee, Break::None, st, None, depth, Some(at));
    let mut b = assemble(alloc::vec![p]);
    JoinSplit { wired: b.wired, witness: b.witness, intent: b.intents.remove(0) }
}
