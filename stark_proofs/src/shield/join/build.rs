// NONOS Operating System (AGPL-3.0-or-later)

use super::parts::{intent_parts, intent_parts_anchored, Spend};
use super::pool::Witnessed;
use super::settle::Settle;
use super::stack::Anchor;
use crate::crypto::stark::air::{WiredMultiGen, RATE};
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
    join_split_at(
        TREE_DEPTH,
        inputs,
        outputs,
        public_amount,
        fee,
        brk,
        st,
        flip,
    )
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
    JoinSplit {
        wired: b.wired,
        witness: b.witness,
        intent: b.intents.remove(0),
    }
}

/// A spend against a pool that already exists.
///
/// The difference from [`join_split`] is the whole difference between a fixture
/// and a transaction. That one plants a tree holding the two notes it is about
/// to spend and proves against the root it just computed, so the statement is
/// true of a pool nobody runs. This one is handed the root the pool published
/// and an opening per input, and proves the notes are in that tree.
///
/// Directions are derived from each position inside the builder rather than
/// passed in, so a caller cannot supply a position and a path that disagree,
/// which is the pair that retires a note under a position the pool never
/// authenticated.
#[allow(clippy::too_many_arguments)]
/// The production spend: notes opened against the pool's published root and
/// the association set opened against the registry's. Everything a
/// `settleBatch` checks before it looks at the proof comes from the caller
/// here, so a spend cannot be built against a root no one issued.
#[allow(clippy::too_many_arguments)]
pub fn join_split_published(
    depth: usize,
    inputs: [Spend; 2],
    openings: [&Witnessed; 2],
    root: [Fp; RATE],
    assoc: super::stack::AssocAnchor<'_>,
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    st: Settle,
    flip: Option<usize>,
) -> JoinSplit {
    let p = intent_parts_anchored(
        inputs,
        outputs,
        public_amount,
        fee,
        brk,
        st,
        flip,
        depth,
        Anchor::Published {
            openings,
            root,
            assoc: Some(assoc),
        },
    );
    let mut b = assemble(alloc::vec![p]);
    JoinSplit {
        wired: b.wired,
        witness: b.witness,
        intent: b.intents.remove(0),
    }
}

pub fn join_split_with_paths(
    depth: usize,
    inputs: [Spend; 2],
    openings: [&Witnessed; 2],
    root: [Fp; RATE],
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    st: Settle,
    flip: Option<usize>,
) -> JoinSplit {
    let p = intent_parts_anchored(
        inputs,
        outputs,
        public_amount,
        fee,
        brk,
        st,
        flip,
        depth,
        Anchor::Published {
            openings,
            root,
            assoc: None,
        },
    );
    let mut b = assemble(alloc::vec![p]);
    JoinSplit {
        wired: b.wired,
        witness: b.witness,
        intent: b.intents.remove(0),
    }
}
