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

//! The streaming trace commitment, the pass that keeps the prover inside a laptop. The trace
//! lives as coefficients; this extends it one coset at a time, hashes each extended row into a
//! Merkle tree, and drops the coset before the next, so the working set is one coset rather
//! than the whole low-degree extension. The extension is tens of terabytes if materialized for
//! the settlement circuit and is never materialized. The tree it returns is the object the
//! transcript absorbs and the queries open.

use super::super::super::field::Fp;
use super::super::super::merkle::hash_leaf_wide;
use super::super::super::merkle::MerkleTree;
use super::coset::extend;
use super::setup::Domain;
use alloc::vec::Vec;

/// The row-wise trace commitment, one coset in memory at a time.
///
/// Leaf `j` hashes every column's value at position `j`, exactly as
/// `commit_wide` does over materialized columns. A coset holds the positions
/// `c + blowup * i`, so its rows land at strides of `blowup` in the digest
/// layer, and after the last coset every leaf has been written once. The tree
/// above the digests is the same `build` either path uses, so the root cannot
/// tell which committer ran.
///
/// The coset stays column-major and each leaf gathers its own row as it is
/// hashed.
///
/// Transposing the coset first looks like the better shape and is not. A
/// trace leaf reads its row exactly once, so a transpose is the same gather
/// plus a full extra write pass and a barrier between the two, and the
/// transpose is memory bound where the gather overlaps with hashing.
/// Measured on the box at the settlement outer's shape: 1,815 seconds
/// transposed against 840 gathered, at 31 of 56 cores against 45. A bench at
/// a short trace length put the two within 12 per cent of each other, which
/// is what a working set that fits in cache will tell you.
pub(in crate::air) fn wide_streamed(coeffs: &[Vec<Fp>], d: &Domain) -> MerkleTree {
    let mut digests = alloc::vec![[0u8; 32]; d.n];
    for c in 0..d.blowup {
        let cols = extend(coeffs, d, c);
        let hashed: Vec<[u8; 32]> = crate::par::map_index(d.t, |i| {
            let row: Vec<Fp> = cols.iter().map(|col| col[i]).collect();
            hash_leaf_wide(&row)
        });
        for (i, h) in hashed.into_iter().enumerate() {
            digests[c + d.blowup * i] = h;
        }
    }
    let pad = alloc::vec![Fp::ZERO; d.width];
    MerkleTree::from_leaf_digests(digests, hash_leaf_wide(&pad))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::air::spec::AirExt;

    /// The committer, timed at the settlement outer's width and blowup over
    /// a shortened trace. Run with `--ignored --nocapture --features
    /// parallel`.
    ///
    /// Read it as a regression line and not as a verdict between shapes: at
    /// this trace length the coset fits in cache, and the transposed
    /// committer this one replaced was within 12 per cent here while being
    /// more than twice as slow on the box. A shape question about memory has
    /// to be asked at the size that has the memory problem.
    #[test]
    #[ignore]
    #[cfg(feature = "parallel")]
    fn the_committer_throughput() {
        use crate::field::Fp;
        use crate::fri::root_of_unity;

        /*
         * The settlement outer's width and window over a shortened trace.
         * The committer's cost per coset is the width times one transform of
         * the trace length, and the shape of that work, not its absolute
         * size, is what the two forms differ on. Sixteen cosets rather than
         * the deployed two hundred and fifty six, so this is seconds.
         */
        let width = 762usize;
        let (log_t, blowup) = (14u32, 16usize);
        let t = 1usize << log_t;
        let log_n = log_t + blowup.trailing_zeros();
        let omega = root_of_unity(log_n);
        let d = Domain {
            t,
            n: 1usize << log_n,
            width,
            window: 2,
            blowup,
            fri_log_blowup: 1,
            g: root_of_unity(log_t),
            omega,
            shift: Fp::from_u64(7),
            sub: omega.pow(blowup as u64),
        };
        let coeffs: Vec<Vec<Fp>> = (0..width)
            .map(|c| {
                (0..d.t)
                    .map(|i| {
                        Fp::from_u64((c as u64 * 1_000_003 + i as u64 * 7 + 1) % 1_000_000_007)
                    })
                    .collect()
            })
            .collect();

        let t0 = std::time::Instant::now();
        let tree = wide_streamed(&coeffs, &d);
        std::println!(
            "COMMIT t=2^{} width={width} blowup={}: {:.3}s (root byte {})",
            d.t.trailing_zeros(),
            d.blowup,
            t0.elapsed().as_secs_f64(),
            tree.root()[0]
        );
    }
}
