// NONOS Operating System (AGPL-3.0-or-later)
//! The note-commitment circuit: prove in-circuit that a shielded note's
//! commitment is the one the pool computes on-chain, without revealing the note.
//!
//! `ShieldedPool._computeCommitment` packs a note into eleven Goldilocks limbs
//! (value low/high, asset id, four spend-key limbs, four blinding limbs) and
//! hashes them with `commitNote`. That is a domain-separated compress tree:
//! the eleven limbs plus `NOTE_DOMAIN` and four zeros form four rate-sized
//! quads, and
//!
//! ```text
//!   d0 = compress(q0, q1)
//!   d1 = compress(q2, q3)
//!   cm = compress(d0, d1)
//! ```
//!
//! A Poseidon compression is exactly one `MultiMembership` level: its `inject`
//! places the node in the low lanes and the sibling in the high lanes, which is
//! the compression input. So the tree is three depth-one openings in one
//! membership region, and the arithmetic that authenticates a Merkle path
//! already carries the note hash. The two internal edges (`d0` and `d1` feeding
//! the final compression) are NOT implied by that region: each opening is
//! independent, so the edges are pinned by explicit copy constraints. Without
//! them a prover could open three unrelated compressions and call the last one a
//! commitment.
//!
//! The hasher here is the POOL's 32-round Poseidon, the one
//! `spec/poseidon-constants.json` pins and `PoseidonGoldilocks.sol` implements,
//! not the recursion's internal hasher. The gate below is a known-answer test
//! against `Poseidon::commit_note`, which is itself gated against the deployed
//! Solidity, so a satisfying trace commits to the same note the pool does.

use crate::crypto::stark::air::{
    stark_prove_ext, stark_verify_ext, Air, GpGroup, MultiMembership, Opening, Poseidon,
    WiredMultiExt, NOTE_DOMAIN, NOTE_LIMBS, RATE,
};
use crate::crypto::stark::field::Fp;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// The pool's hash: `1 << 5` = 32 rounds, matching `FULL_ROUNDS` on-chain.
pub(crate) const POOL_LOG_ROUNDS: u32 = 5;

/// One note, in the limb order `ShieldedPool` packs.
#[derive(Clone, Copy)]
pub(crate) struct Note {
    pub value: u64,
    pub asset_id: u64,
    pub spend_pk: [u64; 4],
    pub blinding: [u64; 4],
}

impl Note {
    /// The eleven limbs, packed exactly as `_computeCommitment` does: the value
    /// split low-32 then high, the asset id, then the two digests' limbs.
    pub fn limbs(&self) -> [Fp; NOTE_LIMBS] {
        let mut l = [Fp::ZERO; NOTE_LIMBS];
        l[0] = Fp::from_u64(self.value & 0xFFFF_FFFF);
        l[1] = Fp::from_u64(self.value >> 32);
        l[2] = Fp::from_u64(self.asset_id);
        for i in 0..4 {
            l[3 + i] = Fp::from_u64(self.spend_pk[i]);
            l[7 + i] = Fp::from_u64(self.blinding[i]);
        }
        l
    }
}

/// The four rate-sized quads the compress tree consumes: the limbs, then the
/// domain separator, then zero padding.
fn quads(limbs: &[Fp; NOTE_LIMBS]) -> [[Fp; RATE]; 4] {
    let mut p = [Fp::ZERO; 16];
    p[..NOTE_LIMBS].copy_from_slice(limbs);
    p[NOTE_LIMBS] = Fp::from_u64(NOTE_DOMAIN);
    let mut q = [[Fp::ZERO; RATE]; 4];
    for (i, qi) in q.iter_mut().enumerate() {
        qi.copy_from_slice(&p[i * RATE..(i + 1) * RATE]);
    }
    q
}

pub(crate) struct NoteCommit {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
    /// The commitment the trace proves, the value a caller publishes.
    pub cm: [Fp; RATE],
}

/// The note circuit as composable parts, for stacking inside a larger circuit.
/// A join-split needs four of these in one proof, so it takes the region and
/// emits the edge constraints itself at the right offsets rather than nesting
/// four finished circuits.
pub(crate) struct NoteParts {
    pub region: MultiMembership,
    pub trace: Vec<Fp>,
    /// Rows per opening, the stride the edge rows derive from.
    pub span_op: usize,
    pub cm: [Fp; RATE],
}

/// The internal edges of a note circuit based at row `base`: the first two
/// compressions feed the third. Returned as `(row, col, row, col)` swaps so the
/// caller can place them in whatever group layout it is building. Lane by lane,
/// because a digest is four field elements and binding one lane would leave the
/// other three free.
pub(crate) fn note_edges(base: usize, span_op: usize) -> Vec<(usize, usize, usize, usize)> {
    let l = 1usize << POOL_LOG_ROUNDS;
    let root_row = |o: usize| base + o * span_op + l; // depth one
    let c_first = base + 2 * span_op;
    let mut sw = Vec::with_capacity(2 * RATE);
    for c in 0..RATE {
        // d0 is the final compression's left input, d1 its right input.
        sw.push((root_row(0), c, c_first, c));
        sw.push((root_row(1), c, c_first, RATE + c));
    }
    sw
}

pub(crate) fn note_parts(note: &Note) -> NoteParts {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let q = quads(&note.limbs());
    let d0 = h.compress(&q[0], &q[1]);
    let d1 = h.compress(&q[2], &q[3]);
    let cm = h.compress(&d0, &d1);

    let one = |leaf: [Fp; RATE], sib: [Fp; RATE], root: [Fp; RATE]| Opening {
        leaf,
        root,
        siblings: alloc::vec![sib],
        directions: alloc::vec![false],
    };
    let region = MultiMembership::new_witness(
        h,
        POOL_LOG_ROUNDS,
        alloc::vec![one(q[0], q[1], d0), one(q[2], q[3], d1), one(d0, d1, cm)],
    );
    let trace = region.trace();
    let span_op = region.opened_cells()[1].0;
    NoteParts { region, trace, span_op, cm }
}

/// A copy constraint over `span` rows: the identity on the listed columns with
/// the named cells transposed, so a satisfying grand product forces them equal.
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

/// Assemble the circuit for `note`. `tamper_edge` breaks the `d0` edge so the
/// reject gate exercises the copy constraint rather than the hash.
pub(crate) fn note_commit(note: &Note, tamper_edge: bool) -> NoteCommit {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let limbs = note.limbs();
    let q = quads(&limbs);

    let d0 = h.compress(&q[0], &q[1]);
    let d1 = h.compress(&q[2], &q[3]);
    let cm = h.compress(&d0, &d1);

    // The final compression's left input. Tampering it keeps every compression
    // internally honest and breaks only the edge, which is the constraint under
    // test.
    let mut c_leaf = d0;
    if tamper_edge {
        c_leaf[0] = c_leaf[0] + Fp::ONE;
    }
    let c_root = if tamper_edge { h.compress(&c_leaf, &d1) } else { cm };

    let one = |leaf: [Fp; RATE], sib: [Fp; RATE], root: [Fp; RATE]| Opening {
        leaf,
        root,
        siblings: alloc::vec![sib],
        directions: alloc::vec![false],
    };
    let openings = alloc::vec![
        one(q[0], q[1], d0),      // A: d0 = compress(q0, q1)
        one(q[2], q[3], d1),      // B: d1 = compress(q2, q3)
        one(c_leaf, d1, c_root),  // C: cm = compress(d0, d1)
    ];

    let region = MultiMembership::new_witness(h.clone(), POOL_LOG_ROUNDS, openings);
    let trace = region.trace();
    let ocells = region.opened_cells();
    let region_width = region.trace_width();

    // Layout: openings are laid out span rows apart, so the second opened cell's
    // row IS the span. A depth-one opening checkpoints its root one slot in.
    let span_op = ocells[1].0;
    let l = 1usize << POOL_LOG_ROUNDS;
    let depth = 1usize;
    let root_row = |o: usize| o * span_op + depth * l;
    // A depth-one opening never writes its level-zero sibling to the witness
    // path columns: `initial_state` injects it directly into the high lanes of
    // the opening's first row. So the final compression's two inputs are the low
    // and high halves of row `2 * span_op`, not a leaf cell and a sibling cell.
    let c_first = 2 * span_op;

    let regions: Vec<Box<dyn crate::crypto::stark::air::AirExt>> = alloc::vec![Box::new(region)];
    let rows = trace.len() / region_width;
    let wired_span = rows.next_power_of_two();

    let mut groups: Vec<GpGroup> = Vec::new();
    // Edge 1: A's root IS the final compression's LEFT input (low lanes).
    for c in 0..RATE {
        groups.push(equate(wired_span, alloc::vec![c], &[(root_row(0), c, c_first, c)]));
    }
    // Edge 2: B's root IS the final compression's RIGHT input (high lanes).
    for c in 0..RATE {
        groups.push(equate(
            wired_span,
            alloc::vec![c, RATE + c],
            &[(root_row(1), c, c_first, RATE + c)],
        ));
    }

    let wired = WiredMultiExt::new(regions, groups);
    let witness = wired.trace(&[trace]);
    NoteCommit { wired, witness, cm: c_root }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Note {
        Note {
            value: 1_234_567_890_123,
            asset_id: 0,
            spend_pk: [11, 22, 33, 44],
            blinding: [55, 66, 77, 88],
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

    /// THE bridge: the circuit's commitment is the pool's commitment. The KAT is
    /// `Poseidon::commit_note`, which `spec/poseidon-constants.json` pins and
    /// `PoseidonGoldilocks.sol` is gated against, so this ties the circuit to the
    /// deployed hasher rather than to a local reimplementation.
    #[test]
    fn commitment_matches_the_pool_hash() {
        let n = sample();
        let asm = note_commit(&n, false);
        let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
        assert_eq!(asm.cm, h.commit_note(&n.limbs()), "circuit cm != commit_note KAT");
    }

    /// The pool hash is FROZEN: this digest is the one in
    /// `spec/poseidon-constants.json`, which `PoseidonGoldilocks.sol` is gated
    /// against and which every live note commitment already depends on.
    ///
    /// It is pinned here, in the prover repo, on purpose. The rest of the suite
    /// only checks that `commit_note` is deterministic and binding, which any
    /// self-consistent hash satisfies — so a change to the classic Poseidon
    /// (say, swapping its linear layer while introducing Poseidon2 for the
    /// recursion) would pass every other Rust test and surface only when the
    /// Solidity gate is re-run in another repo, after notes exist. That failure
    /// must happen here, immediately, and loudly.
    ///
    /// If this test fails, the deployed note commitment has changed. Do not
    /// re-baseline it: introduce the new permutation as a NEW type and leave
    /// `Poseidon` byte-identical.
    #[test]
    fn pool_hash_is_frozen_to_the_deployed_kat() {
        let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
        let mut limbs = [Fp::ZERO; NOTE_LIMBS];
        for (i, l) in limbs.iter_mut().enumerate() {
            *l = Fp::from_u64(i as u64 + 1);
        }
        let want = [
            Fp::from_u64(6455909588408588117),
            Fp::from_u64(11340027322162162298),
            Fp::from_u64(9042362242223743603),
            Fp::from_u64(14573159163843564693),
        ];
        assert_eq!(h.commit_note(&limbs), want, "the deployed pool note hash changed");
    }

    #[test]
    fn honest_note_satisfies_every_constraint() {
        let asm = note_commit(&sample(), false);
        assert!(satisfies(&asm.wired, &asm.witness), "the honest note must satisfy the circuit");
    }

    /// The edges are the whole point: three honest compressions that are not
    /// chained do not commit to a note. Each sub-hash here is internally valid,
    /// so only the copy constraint can catch it.
    #[test]
    fn broken_internal_edge_rejects() {
        let asm = note_commit(&sample(), true);
        assert!(!satisfies(&asm.wired, &asm.witness), "an unchained compress tree must reject");
    }

    #[test]
    fn commitment_is_sensitive_to_every_limb() {
        let base = note_commit(&sample(), false).cm;
        let mut n = sample();
        n.value += 1;
        assert_ne!(note_commit(&n, false).cm, base, "value");
        let mut n = sample();
        n.asset_id += 1;
        assert_ne!(note_commit(&n, false).cm, base, "asset_id");
        for i in 0..4 {
            let mut n = sample();
            n.spend_pk[i] += 1;
            assert_ne!(note_commit(&n, false).cm, base, "spend_pk");
            let mut n = sample();
            n.blinding[i] += 1;
            assert_ne!(note_commit(&n, false).cm, base, "blinding");
        }
    }

    #[test]
    #[ignore]
    fn note_commit_fri_roundtrips() {
        let asm = note_commit(&sample(), false);
        let proof = stark_prove_ext(&asm.wired, &asm.witness, 32, 8);
        assert!(stark_verify_ext(&asm.wired, &proof, 32, 8), "honest note proof did not verify");
    }
}
