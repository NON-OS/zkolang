// NONOS Operating System (AGPL-3.0-or-later)

use super::intent::publics_region;
use super::settle::Settle;
use super::stack::stack;
use super::terms::balance;
use crate::crypto::stark::air::{AirExt, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::key::Break;
use crate::shield::note::{Note, POOL_LOG_ROUNDS};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// balance, four notes, two memberships, two key hierarchies, the tuple.
pub(crate) const REGIONS_PER_INTENT: usize = 14;

pub(crate) struct IntentParts {
    pub regions: Vec<Box<dyn AirExt>>,
    pub traces: Vec<Vec<Fp>>,
    pub span_op: usize,
    pub leaf_col: Vec<usize>,
    pub key_span: Vec<usize>,
    pub assoc_col: Vec<usize>,
    pub depth: usize,
    pub intent: Vec<Fp>,
}

pub(crate) struct Spend<'a> {
    pub note: &'a Note,
    pub sk: [Fp; RATE],
}

pub(crate) fn intent_parts(
    inputs: [Spend; 2],
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    st: Settle,
    flip: Option<usize>,
    depth: usize,
    at: Option<&super::witness::Places>,
) -> IntentParts {
    let notes = [inputs[0].note, inputs[1].note, outputs[0], outputs[1]];
    let values = [notes[0].value, notes[1].value, notes[2].value, notes[3].value];
    let (air, trace) = balance(&values, public_amount, fee);

    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let sks = [inputs[0].sk, inputs[1].sk];
    let mut s = stack(&h, notes, sks, brk, (Box::new(air), trace), depth, at);

    let (intent, pub_air) =
        publics_region(&s, public_amount, fee, notes[0].asset_id, st, flip);
    s.traces.push(pub_air.trace());
    s.regions.push(Box::new(pub_air));

    IntentParts {
        regions: s.regions,
        traces: s.traces,
        span_op: s.span_op,
        leaf_col: s.leaf_col,
        key_span: s.key_span,
        assoc_col: s.assoc_col,
        depth: s.depth,
        intent,
    }
}
