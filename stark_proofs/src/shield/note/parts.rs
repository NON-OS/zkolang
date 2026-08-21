// NONOS Operating System (AGPL-3.0-or-later)

use super::limbs::{quads, Note, POOL_LOG_ROUNDS};
use crate::crypto::stark::air::{MultiMembership, Opening, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(crate) struct NoteParts {
    pub region: MultiMembership,
    pub trace: Vec<Fp>,
    pub span_op: usize,
    pub cm: [Fp; RATE],
}

/// commit_note is a depth two compress tree, and a compression is one membership
/// level, so the note hash is three depth one openings. The edges that chain them
/// are not implied by the region; see note::edges.
pub(crate) fn note_parts(note: &Note) -> NoteParts {
    note_parts_broken(note, false)
}

pub(crate) fn note_parts_broken(note: &Note, break_edge: bool) -> NoteParts {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let q = quads(&note.limbs());
    let d0 = h.compress(&q[0], &q[1]);
    let d1 = h.compress(&q[2], &q[3]);
    // Each compression stays internally honest; only the chain breaks.
    let left = if break_edge {
        let mut x = d0;
        x[0] = x[0] + Fp::ONE;
        x
    } else {
        d0
    };
    let cm = h.compress(&left, &d1);

    let one = |leaf: [Fp; RATE], sib: [Fp; RATE], root: [Fp; RATE]| Opening {
        leaf,
        root,
        siblings: alloc::vec![sib],
        directions: alloc::vec![false],
    };
    let region = MultiMembership::new_witness(
        h,
        POOL_LOG_ROUNDS,
        alloc::vec![one(q[0], q[1], d0), one(q[2], q[3], d1), one(left, d1, cm)],
    );
    let trace = region.trace();
    let span_op = region.opened_cells()[1].0;
    NoteParts {
        region,
        trace,
        span_op,
        cm,
    }
}
