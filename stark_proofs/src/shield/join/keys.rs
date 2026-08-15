// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{AirExt, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::key::{nullifier_parts, Break};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Keys {
    pub regions: Vec<Box<dyn AirExt>>,
    pub traces: Vec<Vec<Fp>>,
    pub key_span: Vec<usize>,
    pub nf: [[Fp; RATE]; 2],
}

pub(crate) fn key_hierarchies(
    sks: [[Fp; RATE]; 2],
    cms: &[[Fp; RATE]; 2],
    leaves: &[usize],
    brk: Break,
) -> Keys {
    let mut regions: Vec<Box<dyn AirExt>> = Vec::with_capacity(2);
    let mut traces = Vec::with_capacity(2);
    let mut key_span = Vec::with_capacity(2);
    let mut nf = [[Fp::ZERO; RATE]; 2];
    for i in 0..2 {
        let k = nullifier_parts(sks[i], cms[i], leaves[i] as u64, brk);
        key_span.push(k.span_op);
        nf[i] = k.nf;
        regions.push(Box::new(k.region));
        traces.push(k.trace);
    }
    Keys { regions, traces, key_span, nf }
}
