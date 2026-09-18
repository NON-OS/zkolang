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

//! Building a capsule's attestation trailer: the prover side an enrollment tool
//! runs. Measure the images to the policy root, prove that one image's
//! measurement sits under it, bound to its context at money-grade soundness, and
//! pack the magic, path, and proof in the exact byte layout the kernel spawn gate
//! parses. The root is not shipped; the kernel holds its own, and enrollment
//! publishes the same one. The leaf is not shipped either: the verifier measures
//! the image it is about to run and pins that, so the proof is about that image.

use super::super::field::Fp;
use super::super::poseidon_merkle::PoseidonMerkleTree;
use super::measure::{measure_capsule, measure_capsule_hybrid};
use super::poseidon::{Poseidon, RATE};
use super::prove_ext::stark_prove_ext_blown_bound;
use super::serialize_ext::serialize_proof_ext;
use super::{MultiMembership, Opening};
use alloc::vec::Vec;

const MAGIC: &[u8; 8] = b"NZKSTRK1";

/// The measured image set, committed once. Every trailer opens the same tree,
/// so an enrollment that measured per capsule would pay for the set N times.
pub struct MeasuredSet {
    leaves: Vec<[Fp; RATE]>,
    tree: PoseidonMerkleTree,
}

impl MeasuredSet {
    /// Measure every image directly into the sponge and commit them under one
    /// tree. A sponge over every byte of every image, and not what the gates
    /// verify: they measure the hybrid way, and the two are domain separated,
    /// so a root built here verifies no gate trailer. It serves the
    /// private-leaf attestation in `attest.rs` and the tests that pin it.
    pub fn commit(hasher: &Poseidon, images: &[&[u8]]) -> MeasuredSet {
        let leaves = images.iter().map(|i| measure_capsule(hasher, i)).collect();
        Self::from_leaves(hasher, leaves)
    }

    /// Measure every image through BLAKE3 and commit the digests under one
    /// tree. This is the scheme the spawn gate and the boot chain verify.
    pub fn commit_hybrid(hasher: &Poseidon, images: &[&[u8]]) -> MeasuredSet {
        let leaves = images
            .iter()
            .map(|i| measure_capsule_hybrid(hasher, i))
            .collect();
        Self::from_leaves(hasher, leaves)
    }

    fn from_leaves(hasher: &Poseidon, leaves: Vec<[Fp; RATE]>) -> MeasuredSet {
        let tree = PoseidonMerkleTree::commit(hasher, &leaves);
        MeasuredSet { leaves, tree }
    }

    /// The policy root the kernel holds.
    pub fn root(&self) -> [Fp; RATE] {
        self.tree.root()
    }

    /// Slot `i`'s measured digest, or `None` past the end of the set.
    pub fn leaf(&self, i: usize) -> Option<[Fp; RATE]> {
        self.leaves.get(i).copied()
    }

    /// Slot `i`'s sibling path, or `None` past the end of the set.
    pub fn path(&self, i: usize) -> Option<Vec<[Fp; RATE]>> {
        (i < self.leaves.len()).then(|| self.tree.open(i))
    }
}

/// The attestation trailer for `images[index]`, bound to `context`, at the given
/// soundness. Measures the set the hybrid way, which is what the gates verify.
#[allow(clippy::too_many_arguments)]
pub fn build_attestation_trailer(
    hasher: &Poseidon,
    log_rounds: u32,
    images: &[&[u8]],
    index: usize,
    context: &[u8],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> Vec<u8> {
    let set = MeasuredSet::commit_hybrid(hasher, images);
    build_attestation_trailer_from_set(
        hasher,
        log_rounds,
        &set,
        index,
        context,
        n_queries,
        grind_bits,
        extra_blowup_bits,
    )
}

/// The same trailer, from a set measured once. Byte layout: magic, depth, the
/// sibling path (four little-endian words per node), the direction bits, then
/// the serialized proof. An enrollment proving many capsules should commit once
/// and call this per capsule.
#[allow(clippy::too_many_arguments)]
pub fn build_attestation_trailer_from_set(
    hasher: &Poseidon,
    log_rounds: u32,
    set: &MeasuredSet,
    index: usize,
    context: &[u8],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> Vec<u8> {
    let path = set.tree.open(index);
    let depth = path.len();
    let directions: Vec<bool> = (0..depth).map(|k| (index >> k) & 1 == 1).collect();

    // The same single opening the verifier rebuilds, leaf pinned with the root.
    let opening = Opening {
        leaf: set.leaves[index],
        root: set.tree.root(),
        siblings: path.clone(),
        directions,
    };
    let air = MultiMembership::new(hasher.clone(), log_rounds, alloc::vec![opening]);
    let trace = air.trace();
    let proof = stark_prove_ext_blown_bound(
        &air,
        &trace,
        n_queries,
        grind_bits,
        extra_blowup_bits,
        context,
    );

    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.push(depth as u8);
    for node in &path {
        for lane in node {
            out.extend_from_slice(&lane.value().to_le_bytes());
        }
    }
    let mut dirs = alloc::vec![0u8; depth.div_ceil(8)];
    for k in 0..depth {
        if (index >> k) & 1 == 1 {
            dirs[k / 8] |= 1 << (k % 8);
        }
    }
    out.extend_from_slice(&dirs);
    out.extend_from_slice(&serialize_proof_ext(&proof));
    out
}
