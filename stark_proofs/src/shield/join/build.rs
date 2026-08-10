// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::{groups, Layout};
use super::bind_publics::public_groups;
use super::intent::publics_region;
use super::settle::Settle;
use super::stack::stack;
use super::terms::balance;
use crate::crypto::stark::air::{Air, Poseidon, WiredMultiExt, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::key::Break;
use crate::shield::note::{Note, POOL_LOG_ROUNDS};
use crate::shield::wire::offsets;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct JoinSplit {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
    pub intent: Vec<Fp>,
}

pub(crate) struct Spend<'a> {
    pub note: &'a Note,
    pub sk: [Fp; RATE],
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
    let notes = [inputs[0].note, inputs[1].note, outputs[0], outputs[1]];
    let values = [notes[0].value, notes[1].value, notes[2].value, notes[3].value];
    let (air, trace) = balance(&values, public_amount, fee);

    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let sks = [inputs[0].sk, inputs[1].sk];
    let mut s = stack(&h, notes, sks, brk, (Box::new(air), trace));

    let (intent, pub_air) =
        publics_region(&s, public_amount, fee, notes[0].asset_id, st, flip);
    s.traces.push(pub_air.trace());
    s.regions.push(Box::new(pub_air));

    let rows: Vec<usize> = s.regions.iter().map(|r| 1usize << r.log_trace_len()).collect();
    let (off, span) = offsets(&rows);
    let lay = Layout {
        span,
        span_op: s.span_op,
        note: off[1..5].to_vec(),
        member: off[5..7].to_vec(),
        key: off[7..9].to_vec(),
        key_span: s.key_span,
        leaf_col: s.leaf_col,
        balance: off[0],
    };
    let mut g = groups(&lay);
    g.extend(public_groups(&lay, off[9]));
    let wired = WiredMultiExt::new(s.regions, g);
    let witness = wired.trace(&s.traces);
    JoinSplit { wired, witness, intent }
}
