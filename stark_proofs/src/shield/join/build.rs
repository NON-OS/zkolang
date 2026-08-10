// NONOS Operating System (AGPL-3.0-or-later)

use super::parts::{intent_parts, Spend};
use super::settle::Settle;
use crate::crypto::stark::air::WiredMultiExt;
use crate::crypto::stark::field::Fp;
use crate::shield::batch::assemble;
use crate::shield::key::Break;
use crate::shield::note::Note;
use alloc::vec::Vec;

pub(crate) struct JoinSplit {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
    pub intent: Vec<Fp>,
}

/// One intent is a batch of one, so the layout lives in a single place.
pub(crate) fn join_split(
    inputs: [Spend; 2],
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    st: Settle,
    flip: Option<usize>,
) -> JoinSplit {
    let p = intent_parts(inputs, outputs, public_amount, fee, brk, st, flip);
    let mut b = assemble(alloc::vec![p]);
    JoinSplit { wired: b.wired, witness: b.witness, intent: b.intents.remove(0) }
}
