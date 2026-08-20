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
    /// A different secret behind the nullifier key: the two derivations no
    /// longer share one sk.
    ForeignKey,
    /// Absorb a commitment other than the one membership proved.
    ForeignNote,
    /// Spend a note whose committed key this secret did not derive.
    NotOwner,
    /// Three honest compressions that are not chained into a commitment.
    NoteEdge,
    /// Prove association membership of a note other than the one spent.
    Unlisted,
    /// Retire the note under a leaf index other than the one the pool authenticated.
    /// The note is genuinely owned and genuinely a member; only the position the
    /// nullifier hashes moves. Two of these over one note are two nullifiers, and
    /// two nullifiers over one note is that note spent twice.
    ForeignIndex,
    /// Walk the second note's membership to a pool of its own. Only the first
    /// note's walked root is compared to the published one, so the second walks
    /// to a root nobody named and the note need not be in the pool at all.
    ForeignPoolRoot,
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
    // Flipping the lowest bit alone is the sharp case: it is the bit the opening
    // consumes building its initial state, so it is the last one to become a cell
    // a binding can reach.
    let idx = if brk == Break::ForeignIndex { leaf_index ^ 1 } else { leaf_index };
    let t = h.compress(&nk, &target);
    let nf = h.compress(&t, &tag(idx));

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
            one(t, tag(idx), nf),
        ],
    );
    let trace = region.trace();
    let span_op = region.opened_cells()[1].0;
    NullifierParts { region, trace, span_op, nf, spend_pk }
}
