// NONOS Operating System (AGPL-3.0-or-later)
//! The spend authority half of a join-split: the nullifier, and the secret that
//! ties it to the note being spent.
//!
//! `SPEC.md` §4 fixes the nullifier as `nf = Poseidon(nk, cm, leaf_index)`. That
//! shape alone is not enough. If `nk` enters as a free witness, the same `cm`
//! yields a different valid nullifier for every `nk` a prover picks, so a note is
//! spendable repeatedly; and anyone who has merely seen a commitment can invent
//! an `nk` and retire a note they do not own. Both close only when `nk` and the
//! `spend_pk` committed *inside* that `cm` descend from one witnessed secret:
//!
//! ```text
//!   spend_pk = compress(sk, [SPEND_DOMAIN, 0, 0, 0])   committed in the note
//!   nk       = compress(sk, [NULL_DOMAIN,  0, 0, 0])
//!   nf       = compress( compress(nk, cm), [leaf_index, 0, 0, 0] )
//! ```
//!
//! A compression is one `MultiMembership` level, so this is four depth-one
//! openings plus the copy constraints that chain them. The chaining is the whole
//! statement: four honest compressions that are not tied together prove nothing
//! about ownership.
//!
//! `leaf_index` is in the preimage on purpose. Two deposits of an identical
//! `(value, assetId, spendPk, blinding)` commit to the same `cm` at different
//! leaves; without the position they would share a nullifier, so spending the
//! first would silently lock the second forever. Anyone "simplifying" this to
//! `compress(nk, cm)` reintroduces a fund-losing bug that no test of the honest
//! path will show.
//!
//! DOMAIN CONSTANTS ARE PROVISIONAL. The structure here is what takes
//! engineering and does not depend on their values, but the wallet and the client
//! must derive byte-identically or every note becomes unspendable. Until §4 is
//! ratified these two values are proposals, and `the_domains_are_provisional`
//! keeps that fact in the build rather than in a thread.

use crate::crypto::stark::air::{
    Air, GpGroup, MultiMembership, Opening, Poseidon, WiredMultiExt, RATE,
};
use crate::crypto::stark::field::Fp;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::note_commit::POOL_LOG_ROUNDS;

/// Proposed in SPEC.md §4, pending wallet-owner ratification. "SPND".
pub(crate) const SPEND_DOMAIN: u64 = 0x5350_4E44;
/// Proposed in SPEC.md §4, pending wallet-owner ratification. "NULL".
pub(crate) const NULL_DOMAIN: u64 = 0x4E55_4C4C;

fn tag(v: u64) -> [Fp; RATE] {
    let mut q = [Fp::ZERO; RATE];
    q[0] = Fp::from_u64(v);
    q
}

/// The key hierarchy, computed the way the wallet will have to compute it.
pub(crate) struct Keys {
    pub spend_pk: [Fp; RATE],
    pub nk: [Fp; RATE],
}

pub(crate) fn derive(h: &Poseidon, sk: [Fp; RATE]) -> Keys {
    Keys { spend_pk: h.compress(&sk, &tag(SPEND_DOMAIN)), nk: h.compress(&sk, &tag(NULL_DOMAIN)) }
}

pub(crate) fn nullifier(h: &Poseidon, nk: [Fp; RATE], cm: [Fp; RATE], leaf_index: u64) -> [Fp; RATE] {
    let t = h.compress(&nk, &cm);
    h.compress(&t, &tag(leaf_index))
}

/// Which tie to cut, so every reject fires through one binding and nothing else.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Break {
    None,
    /// Derive the nullifier key from a different secret than the note's owner.
    ForeignKey,
    /// Nullify a commitment other than the one the key opens.
    ForeignNote,
}

pub(crate) struct Nullifier {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
    pub nf: [Fp; RATE],
    pub spend_pk: [Fp; RATE],
}

fn equate(span: usize, cols: Vec<usize>, swaps: &[(usize, usize, usize, usize)]) -> GpGroup {
    let k = cols.len();
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for &(ra, ca, rb, cb) in swaps {
        let ia = cols.iter().position(|&c| c == ca).unwrap();
        let ib = cols.iter().position(|&c| c == cb).unwrap();
        sigma.swap(ra * k + ia, rb * k + ib);
    }
    GpGroup { wired_cols: cols, sigma, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
}

/// Prove `nf` retires the note `cm` at `leaf_index`, under a secret whose
/// `spend_pk` is the one that note committed to.
pub(crate) fn prove_nullifier(
    sk: [Fp; RATE],
    cm: [Fp; RATE],
    leaf_index: u64,
    brk: Break,
) -> Nullifier {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);

    // A foreign key is a different secret that is internally consistent: every
    // compression below still holds, only the tie to the note's spend_pk breaks.
    let mut nk_sk = sk;
    if brk == Break::ForeignKey {
        nk_sk[0] = nk_sk[0] + Fp::ONE;
    }
    let mut target = cm;
    if brk == Break::ForeignNote {
        target[0] = target[0] + Fp::ONE;
    }

    let spend_pk = h.compress(&sk, &tag(SPEND_DOMAIN));
    let nk = h.compress(&nk_sk, &tag(NULL_DOMAIN));
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
            one(sk, tag(SPEND_DOMAIN), spend_pk), // 0: spend_pk from sk
            one(nk_sk, tag(NULL_DOMAIN), nk),     // 1: nk from the same sk
            one(nk, target, t),                   // 2: nk absorbs the commitment
            one(t, tag(leaf_index), nf),          // 3: the position pins the nullifier
        ],
    );
    let trace = region.trace();
    let width = region.trace_width();
    let span_op = region.opened_cells()[1].0;
    let l = 1usize << POOL_LOG_ROUNDS;
    let rows = trace.len() / width;
    let span = rows.next_power_of_two();

    // A depth-one opening carries its leaf in the low lanes of its first row and
    // its sibling in the high lanes, and checkpoints its root one slot in.
    let first = |o: usize| o * span_op;
    let root = |o: usize| o * span_op + l;

    let mut groups: Vec<GpGroup> = Vec::new();
    for c in 0..RATE {
        // One secret behind both keys. Without this the nullifier key is free and
        // the same note yields a fresh nullifier per key a prover picks.
        groups.push(equate(span, alloc::vec![c], &[(first(0), c, first(1), c)]));
        // The key that absorbs the commitment is the key that secret derived.
        groups.push(equate(span, alloc::vec![c], &[(root(1), c, first(2), c)]));
        // And the nullifier is taken over that absorption, not a free value.
        groups.push(equate(span, alloc::vec![c], &[(root(2), c, first(3), c)]));
    }

    let regions: Vec<Box<dyn crate::crypto::stark::air::AirExt>> = alloc::vec![Box::new(region)];
    let wired = WiredMultiExt::new(regions, groups);
    let witness = wired.trace(&[trace]);
    Nullifier { wired, witness, nf, spend_pk }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note_commit::{note_parts, Note};

    fn hasher() -> Poseidon {
        Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
    }

    fn secret(seed: u64) -> [Fp; RATE] {
        let mut sk = [Fp::ZERO; RATE];
        for (i, v) in sk.iter_mut().enumerate() {
            *v = Fp::from_u64(seed * 16 + i as u64 + 1);
        }
        sk
    }

    /// A note owned by `sk`: its committed spend_pk is the one that secret derives.
    fn owned_note(sk: [Fp; RATE], value: u64) -> Note {
        let k = derive(&hasher(), sk);
        Note {
            value,
            asset_id: 0,
            spend_pk: [
                k.spend_pk[0].value(),
                k.spend_pk[1].value(),
                k.spend_pk[2].value(),
                k.spend_pk[3].value(),
            ],
            blinding: [7, 8, 9, 10],
        }
    }

    fn satisfies(air: &WiredMultiExt, witness: &[Fp]) -> bool {
        let w = air.trace_width();
        let ws = air.window_size();
        let total = 1usize << air.log_trace_len();
        let periodic = air.periodic_columns();
        for r in 0..total - (ws - 1) {
            let mut window = Vec::with_capacity(ws * w);
            for k in 0..ws {
                window.extend_from_slice(&witness[(r + k) * w..(r + k + 1) * w]);
            }
            let per: Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
            if air.transition(&window, &per).iter().any(|v| *v != Fp::ZERO) {
                return false;
            }
        }
        for (col, row, val) in air.boundary() {
            if witness[row * w + col] != val {
                return false;
            }
        }
        true
    }

    #[test]
    fn an_owner_can_retire_their_note() {
        let sk = secret(1);
        let cm = note_parts(&owned_note(sk, 1000)).cm;
        let n = prove_nullifier(sk, cm, 3, Break::None);
        assert!(satisfies(&n.wired, &n.witness), "the owner must be able to spend");
        assert_eq!(n.nf, nullifier(&hasher(), derive(&hasher(), sk).nk, cm, 3));
    }

    /// The derivation is what makes the note the owner's: the spend_pk the circuit
    /// derives is the one sitting in the commitment.
    #[test]
    fn the_derived_key_is_the_one_the_note_committed_to() {
        let sk = secret(2);
        let note = owned_note(sk, 500);
        let n = prove_nullifier(sk, note_parts(&note).cm, 0, Break::None);
        let committed: Vec<u64> = note.spend_pk.to_vec();
        let derived: Vec<u64> = n.spend_pk.iter().map(|v| v.value()).collect();
        assert_eq!(derived, committed, "the circuit derived a key the note does not commit to");
    }

    /// Double spend by alternate key: same note, a different secret behind the
    /// nullifier key. Every compression stays honest, so only the shared-secret
    /// binding can reject.
    #[test]
    fn a_second_key_cannot_produce_a_second_nullifier() {
        let sk = secret(3);
        let cm = note_parts(&owned_note(sk, 900)).cm;
        let n = prove_nullifier(sk, cm, 1, Break::ForeignKey);
        assert!(!satisfies(&n.wired, &n.witness), "a foreign key retired the note");
    }

    /// The scope boundary, asserted so it cannot be forgotten.
    ///
    /// Nullifying a commitment the key does not open is NOT caught here, and the
    /// constraints are satisfied when it happens. This circuit takes `cm` as a
    /// witness and proves the key hierarchy over it; nothing in it says that
    /// `cm` is a real note, nor that the secret owns it. Ownership is the
    /// assembly's cross binding: the derived `spend_pk` must equal the `spend_pk`
    /// committed inside the very `cm` the membership proof authenticated.
    ///
    /// Written as a passing assertion rather than a rejection because a test that
    /// merely rejects would suggest the property lives here. It does not, and a
    /// reader who assumes otherwise ships theft.
    #[test]
    fn a_foreign_note_is_not_rejected_here_ownership_is_the_assemblys_job() {
        let sk = secret(4);
        let cm = note_parts(&owned_note(sk, 250)).cm;
        let n = prove_nullifier(sk, cm, 2, Break::ForeignNote);
        assert!(
            satisfies(&n.wired, &n.witness),
            "if this now rejects, the ownership binding moved into this circuit and \
             the assembly's cross binding needs revisiting"
        );
        // What the circuit does give: a different commitment retires a different
        // note, so the two cannot be confused once ownership is bound.
        let honest = prove_nullifier(sk, cm, 2, Break::None);
        assert_ne!(n.nf, honest.nf, "a foreign note produced the same nullifier");
    }

    /// The position is load-bearing. Identical notes commit identically, so
    /// without the leaf in the preimage they would share a nullifier and spending
    /// one would lock the other for good.
    #[test]
    fn identical_notes_at_different_leaves_have_different_nullifiers() {
        let sk = secret(5);
        let cm = note_parts(&owned_note(sk, 42)).cm;
        let h = hasher();
        let nk = derive(&h, sk).nk;
        assert_ne!(
            nullifier(&h, nk, cm, 0),
            nullifier(&h, nk, cm, 1),
            "the same note at two leaves shares a nullifier, so one of them is unspendable"
        );
    }

    /// The domains are proposals until the key hierarchy is ratified and the
    /// wallet derives against the same vector. They must stay distinct from each
    /// other and from the note domain, or a key doubles as a commitment preimage.
    #[test]
    fn the_domains_are_provisional_but_separated() {
        use crate::crypto::stark::air::NOTE_DOMAIN;
        assert_ne!(SPEND_DOMAIN, NULL_DOMAIN);
        assert_ne!(SPEND_DOMAIN, NOTE_DOMAIN);
        assert_ne!(NULL_DOMAIN, NOTE_DOMAIN);
    }
}
