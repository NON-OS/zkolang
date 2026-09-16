// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The preprocessed-periodic root: the keccak wide-periodic Merkle root over the
//! coset low-degree extension of an AIR's periodic columns. It is the constant a
//! preprocessed-periodic verifier bakes, and the value a per-program verifier key
//! binds to the program that produced the wiring. The preprocessed prover
//! (`prove_ext_pre`) commits the periodic tree through the same function here, so a
//! root computed for registration and a root committed inside a proof are the same
//! object by construction, not by agreement.

use alloc::vec::Vec;

use super::super::field::Fp;
use super::super::merkle::{hash_leaf_wide_periodic, MerkleTree, PeriodicLeafHasher};
use super::composition::domain_params_blown;
use super::prove_ext::{extend, periodic_coeffs, Domain};
use super::spec::AirExt;

/*
 * Where the periodic commitment's memory actually goes.
 *
 * The prover reached this function holding 13.5 GB and was killed above 72,
 * and every estimate of which step asked for the difference has been wrong.
 * These say it rather than model it. Parallel builds only, which is the build
 * that has a standard library to print with.
 */
#[cfg(feature = "parallel")]
fn mark(what: &str) {
    if let Ok(s) = std::fs::read_to_string("/proc/self/statm") {
        let pages: u64 = s
            .split_whitespace()
            .nth(1)
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        std::eprintln!(
            "[periodic] {what}: resident {:.1} GB",
            pages as f64 * 4096.0 / 1e9
        );
    }
}

#[cfg(not(feature = "parallel"))]
fn mark(_what: &str) {}

/// The periodic coefficients and the wide-periodic tree over their coset
/// extension. Both the preprocessed prover and the root helper go through this,
/// so the periodic domain size, the coset, and the leaf and node rules cannot
/// drift between them. The extension itself is never held: leaves are hashed
/// one coset at a time, and the tree above the digests is the same build the
/// materialized committer used, so the root cannot tell which one ran.
pub(super) fn periodic_tree<A: AirExt>(
    air: &A,
    extra_blowup_bits: u32,
) -> (Vec<Vec<Fp>>, MerkleTree) {
    let d = Domain::of(air, extra_blowup_bits);
    periodic_tree_over(air.periodic_columns(), &d)
}

/*
 * The same commitment over columns the caller already holds.
 *
 * A prover that needs the periodic columns for anything else would otherwise
 * build them twice, once for itself and once in here, and on the settlement
 * outer one copy is 2,649 columns of a quarter million elements, about five
 * and a half gigabytes. Taking them by value lets the caller hand over its
 * only copy and lets this function drop it at the same point it always did.
 */
pub(in crate::air) fn periodic_tree_over(
    cols: Vec<Vec<Fp>>,
    d: &Domain,
) -> (Vec<Vec<Fp>>, MerkleTree) {
    let n_cols = cols.len();
    mark("columns built");
    let coeffs = periodic_coeffs(&cols, d);
    mark("interpolated");
    /*
     * The columns are read once, to interpolate, and dropped here. Everything
     * below works from the coefficients, so holding both would keep the whole
     * periodic set twice, and on a wide outer that set is gigabytes.
     */
    drop(cols);
    mark("columns dropped");
    // A columnless AIR commits the empty tree, as the materialized committer
    // always did: it took its leaf count from the columns, this path takes it
    // from the domain, and only the empty case can tell them apart.
    if n_cols == 0 {
        return (coeffs, MerkleTree::commit_wide_periodic(&[]));
    }
    /*
     * Cosets are independent, so the work parallelises across them rather than
     * only inside one. Parallelising only the absorb left a 56 core box under a
     * fifth loaded on the settlement outer, because a column chunk is 64 wide
     * and nothing else is in flight while one coset finishes.
     *
     * In batches, though, and the reason is memory rather than taste. Each
     * coset in flight holds its own leaf states and its own extended chunk, and
     * on the settlement outer that is most of two hundred megabytes apiece.
     * Turning every coset loose at once let the machine ask for tens of
     * gigabytes beyond the coefficients and the kernel killed the prover three
     * hours in. A batch bounds what is live to the batch size, which keeps the
     * speed and gives the peak a ceiling that does not move with the core count.
     *
     * Digest order is untouched. Each coset still walks its rows in the same
     * order and they are scattered to the same positions afterwards, so the
     * tree above them and the root are the ones the sequential walk produced.
     */
    let mut digests = alloc::vec![[0u8; 32]; d.n];
    mark("digest array");
    let mut first = 0usize;
    while first < d.blowup {
        let batch = core::cmp::min(COSET_BATCH, d.blowup - first);
        let rows: Vec<Vec<[u8; 32]>> = crate::par::map_index(batch, |k| {
            let c = first + k;
            /*
             * One incremental leaf hash per row of the coset, the tag already
             * absorbed. The columns then stream past a chunk at a time, each
             * row absorbing its value from each column in column order, which
             * is byte for byte the row the materialised committer hashes
             * whole, so the digest is the same and no row is ever built.
             */
            let mut leaves: Vec<PeriodicLeafHasher> =
                (0..d.t).map(|_| PeriodicLeafHasher::new()).collect();
            for chunk in coeffs.chunks(COLUMN_CHUNK) {
                let ext = extend(chunk, d, c);
                for (i, leaf) in leaves.iter_mut().enumerate() {
                    for col in &ext {
                        leaf.absorb(col[i]);
                    }
                }
            }
            leaves.into_iter().map(|leaf| leaf.finalize()).collect()
        });
        for (k, row) in rows.into_iter().enumerate() {
            let c = first + k;
            for (i, dig) in row.into_iter().enumerate() {
                digests[c + d.blowup * i] = dig;
            }
        }
        first += batch;
        if first % (COSET_BATCH * 8) == 0 {
            mark("cosets");
        }
    }
    mark("cosets done");
    let pad = alloc::vec![Fp::ZERO; n_cols];
    let tree = MerkleTree::from_leaf_digests(digests, hash_leaf_wide_periodic(&pad));
    mark("tree built");
    (coeffs, tree)
}

/// Columns extended per pass. Sized so a chunk over one coset stays in the
/// low hundreds of megabytes on the widest outer.
const COLUMN_CHUNK: usize = 64;

/// Cosets held in flight at once, which is the knob that bounds peak memory
/// and, on a wide machine, the knob that bounds throughput.
///
/// Eight was chosen when a leaf hasher buffered every byte it was ever given,
/// about 21 kB apiece. It no longer does: the struct measures 248 bytes and its
/// one block of buffer never exceeds the rate, so a coset in flight now costs
/// about 132 MB of leaf states and 134 MB of extended chunk, call it 266 MB.
///
/// At eight that is 2 GB and a 56 core box runs at eleven cores, because the
/// absorb inside a coset is serial and only the batch is parallel. At thirty
/// two it is 8.5 GB, which is comfortable against the tens of gigabytes free,
/// and the machine is no longer the thing waiting.
///
/// Digest order does not depend on it: cosets are scattered to fixed positions
/// afterwards, so this changes when work happens and never what is produced.
const COSET_BATCH: usize = 32;

/// The committer as it stood before the leaf hashes streamed: every column held
/// over the coset and a row built per leaf. Kept only as the reference the lean
/// path is checked against, so a change to the streaming can never move the root
/// without a test saying so.
#[cfg(test)]
pub(super) fn periodic_tree_held<A: AirExt>(
    air: &A,
    extra_blowup_bits: u32,
) -> (Vec<Vec<Fp>>, MerkleTree) {
    let d = Domain::of(air, extra_blowup_bits);
    let cols = air.periodic_columns();
    let coeffs = periodic_coeffs(&cols, &d);
    if cols.is_empty() {
        return (coeffs, MerkleTree::commit_wide_periodic(&[]));
    }
    let mut digests = alloc::vec![[0u8; 32]; d.n];
    for c in 0..d.blowup {
        let per = extend(&coeffs, &d, c);
        let hashed: Vec<[u8; 32]> = crate::par::map_index(d.t, |i| {
            let row: Vec<Fp> = per.iter().map(|col| col[i]).collect();
            hash_leaf_wide_periodic(&row)
        });
        for (i, h) in hashed.into_iter().enumerate() {
            digests[c + d.blowup * i] = h;
        }
    }
    let pad = alloc::vec![Fp::ZERO; cols.len()];
    let tree = MerkleTree::from_leaf_digests(digests, hash_leaf_wide_periodic(&pad));
    (coeffs, tree)
}

/// The 32-byte preprocessed-periodic root for `air` at the given FRI rate. This is
/// the value a per-program verifier key binds; the preprocessed prover commits the
/// identical tree, so the two never diverge.
pub fn periodic_root<A: AirExt>(air: &A, extra_blowup_bits: u32) -> [u8; 32] {
    periodic_tree(air, extra_blowup_bits).1.root()
}

/// The log2 of the periodic evaluation domain for `air` at this FRI rate: the
/// size each periodic column is extended to. Deterministic public structure,
/// exposed so an out-of-crate committer (for example a parallel one) sizes the
/// domain identically to the serial path and cannot drift from it.
pub fn periodic_domain_log<A: AirExt>(air: &A, extra_blowup_bits: u32) -> u32 {
    domain_params_blown(air, extra_blowup_bits).0
}

#[cfg(test)]
mod tests {
    use super::super::spec::{Air, AirExt};
    use super::*;
    use crate::field::Fp2;

    /// The zero-column AIR: the one shape that can tell the streamed committer
    /// from the materialized one, because only there do "leaf count from the
    /// domain" and "leaf count from the columns" disagree.
    struct NoPeriodic;

    impl Air for NoPeriodic {
        fn log_trace_len(&self) -> u32 {
            4
        }
        fn trace_width(&self) -> usize {
            1
        }
        fn window_size(&self) -> usize {
            2
        }
        fn constraint_degree(&self) -> usize {
            3
        }
        fn num_transition(&self) -> usize {
            1
        }
        fn periodic_columns(&self) -> Vec<Vec<Fp>> {
            Vec::new()
        }
        fn transition(&self, w: &[Fp], _p: &[Fp]) -> Vec<Fp> {
            alloc::vec![w[1] - w[0]]
        }
        fn boundary(&self) -> Vec<(usize, usize, Fp)> {
            Vec::new()
        }
    }

    impl AirExt for NoPeriodic {
        fn transition_ext(&self, w: &[Fp2], _p: &[Fp2]) -> Vec<Fp2> {
            alloc::vec![w[1] - w[0]]
        }
    }

    /// A leaf absorbed one value at a time is the leaf hashed whole: same tag,
    /// same bytes, same order, same sponge. This is the equality the streaming
    /// committer rests on, checked at the leaf before it is checked at the root.
    #[test]
    fn a_streamed_leaf_equals_the_whole_row_hash() {
        for len in [0usize, 1, 3, 17, 64] {
            let row: Vec<Fp> = (0..len).map(|i| Fp::from_u64(7 * i as u64 + 11)).collect();
            let mut h = PeriodicLeafHasher::new();
            for &v in &row {
                h.absorb(v);
            }
            assert_eq!(
                h.finalize(),
                hash_leaf_wide_periodic(&row),
                "leaf of width {len} moved"
            );
        }
    }

    /// The lean committer, one chunk of columns and the leaf states at a time,
    /// commits the identical tree to the committer that held every column and
    /// built every row. Checked on an AIR with real periodic columns, at the
    /// minimal rate and a raised one, so both the coset decomposition and the
    /// chunking are covered. A root that moved here would move under every
    /// registered verifier key at once.
    #[test]
    fn the_lean_committer_matches_the_held_one() {
        use super::super::index_scalar::IndexScalar;
        let air = IndexScalar::new(6, 5);
        assert!(
            !air.periodic_columns().is_empty(),
            "the test AIR must carry periodic columns"
        );
        for extra in [0u32, 1] {
            let lean = periodic_tree(&air, extra).1.root();
            let held = periodic_tree_held(&air, extra).1.root();
            assert_eq!(
                lean, held,
                "the lean periodic root moved at extra blowup {extra}"
            );
        }
    }

    #[test]
    fn a_columnless_air_commits_the_empty_tree() {
        let (coeffs, tree) = periodic_tree(&NoPeriodic, 1);
        assert!(coeffs.is_empty());
        let empty = MerkleTree::commit_wide_periodic(&[]);
        assert_eq!(
            tree.root(),
            empty.root(),
            "streamed and materialized roots diverge at zero columns"
        );
    }
}
