// NONOS Operating System (AGPL-3.0-or-later)

use super::derive::derive;
use super::domain::tag;
use crate::crypto::stark::air::{MultiMembership, Opening, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::note::POOL_LOG_ROUNDS;
use alloc::vec::Vec;

/// Which tie to cut, so a reject fires through one binding and nothing else.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Break {
    None,
    ForeignKey,
    ForeignNote,
}

pub(crate) struct NullifierParts {
    pub region: MultiMembership,
    pub trace: Vec<Fp>,
    pub span_op: usize,
    pub nf: [Fp; RATE],
    pub spend_pk: [Fp; RATE],
}

pub(crate) fn nullifier_parts(
    sk: [Fp; RATE],
    cm: [Fp; RATE],
    leaf_index: u64,
    brk: Break,
) -> NullifierParts {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let mut nk_sk = sk;
    if brk == Break::ForeignKey {
        nk_sk[0] = nk_sk[0] + Fp::ONE;
    }
    let mut target = cm;
    if brk == Break::ForeignNote {
        target[0] = target[0] + Fp::ONE;
    }

    let spend_pk = derive(&h, sk).spend_pk;
    let nk = derive(&h, nk_sk).nk;
    let t = h.compress(&nk, &target);
    let nf = h.compress(&t, &tag(leaf_index));

    let one = |leaf: [Fp; RATE], sib: [Fp; RATE], root: [Fp; RATE]| Opening {
        leaf,
        root,
        siblings: alloc::vec![sib],
        directions: alloc::vec![false],
    };
    let region = MultiMembership::new_witness(
        h,
        POOL_LOG_ROUNDS,
        alloc::vec![
            one(sk, tag(super::domain::SPEND_DOMAIN), spend_pk),
            one(nk_sk, tag(super::domain::NULL_DOMAIN), nk),
            one(nk, target, t),
            one(t, tag(leaf_index), nf),
        ],
    );
    let trace = region.trace();
    let span_op = region.opened_cells()[1].0;
    NullifierParts { region, trace, span_op, nf, spend_pk }
}
