// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{ShieldRegion, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::key::Break;
use crate::shield::note::{note_parts_broken, Note};
use alloc::vec::Vec;

pub struct Notes {
    pub regions: Vec<ShieldRegion>,
    pub traces: Vec<Vec<Fp>>,
    pub span_op: usize,
    pub cms: Vec<[Fp; RATE]>,
}

pub fn note_regions(notes: [&Note; 4], brk: Break) -> Notes {
    let parts: Vec<_> =
        notes.iter().map(|n| note_parts_broken(n, brk == Break::NoteEdge)).collect();
    let span_op = parts[0].span_op;
    let cms: Vec<[Fp; RATE]> = parts.iter().map(|p| p.cm).collect();
    let mut regions: Vec<ShieldRegion> = Vec::with_capacity(4);
    let mut traces = Vec::with_capacity(4);
    for p in parts {
        regions.push(ShieldRegion::Membership(p.region));
        traces.push(p.trace);
    }
    Notes { regions, traces, span_op, cms }
}
