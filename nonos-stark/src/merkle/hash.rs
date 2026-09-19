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

//! Domain-separated leaf and node hashing for the Merkle commitment. Keccak256 is
//! the hash, already the EVM-native hash, so the on-chain verifier recomputes leaves and nodes with a
//! single native opcode instead of a costly in-Solidity hash.

use super::super::field::{Fp, Fp2};
use crate::hash::Keccak;

const DOM_LEAF: &[u8] = b"NONOS-STARK-MERKLE-LEAF";
const DOM_LEAF_EXT: &[u8] = b"NONOS-STARK-MERKLE-LEAF-EXT";
const DOM_LEAF_WIDE: &[u8] = b"NONOS-STARK-MERKLE-LEAF-WIDE";
const DOM_LEAF_PERIODIC: &[u8] = b"NONOS-STARK-PERIODIC-WIDE";
const DOM_NODE: &[u8] = b"NONOS-STARK-MERKLE-NODE";

/// Hash a field element into a leaf digest, domain-separated from node hashing
/// so a leaf can never be reinterpreted as an internal node.
pub(super) fn hash_leaf(leaf: Fp) -> [u8; 32] {
    let mut k = Keccak::new(512, 32, 0x01);
    k.update(DOM_LEAF);
    k.update(&leaf.value().to_le_bytes());
    k.finalize32()
}

/// Hash an extension-field element into a leaf digest: both lanes under a distinct
/// domain, so a folded FRI layer commits like a base layer but can never be
/// confused with one, an internal node, or a differently-shaped leaf.
pub(super) fn hash_leaf_ext(leaf: Fp2) -> [u8; 32] {
    let mut k = Keccak::new(512, 32, 0x01);
    k.update(DOM_LEAF_EXT);
    k.update(&leaf.c0.value().to_le_bytes());
    k.update(&leaf.c1.value().to_le_bytes());
    k.finalize32()
}

/// Hash a whole trace row into one leaf: every column's canonical value as an
/// 8-byte little-endian integer, tightly packed in column order, under a domain
/// distinct from the single-value leaf so a wide leaf can never be confused with a
/// base leaf, an extension leaf, or an internal node. This is the commitment shape
/// the on-chain verifier recomputes once per query instead of one path per column.
pub fn hash_leaf_wide(row: &[Fp]) -> [u8; 32] {
    let mut k = Keccak::new(512, 32, 0x01);
    k.update(DOM_LEAF_WIDE);
    for v in row {
        k.update(&v.value().to_le_bytes());
    }
    k.finalize32()
}

/// Hash a row of preprocessed periodic-column values into one leaf: identical
/// packing to the wide trace leaf under its own domain, so the structural
/// periodic commitment a verifier bakes as a constant can never be read as a
/// trace leaf.
pub fn hash_leaf_wide_periodic(row: &[Fp]) -> [u8; 32] {
    let mut k = Keccak::new(512, 32, 0x01);
    k.update(DOM_LEAF_PERIODIC);
    for v in row {
        k.update(&v.value().to_le_bytes());
    }
    k.finalize32()
}

/// The wide periodic leaf hash absorbed one value at a time. `hash_leaf_wide_periodic`
/// hashes a row whole; this pushes the same bytes in the same order, the tag and then
/// each value's little endian encoding, through the same sponge, so a leaf built from
/// columns streamed past it in column order finalises to the identical digest. It is
/// what lets a committer hold one chunk of columns at a time instead of all of them.
pub struct PeriodicLeafHasher(Keccak);

impl PeriodicLeafHasher {
    pub fn new() -> PeriodicLeafHasher {
        let mut k = Keccak::new(512, 32, 0x01);
        k.update(DOM_LEAF_PERIODIC);
        PeriodicLeafHasher(k)
    }

    pub fn absorb(&mut self, v: Fp) {
        self.0.update(&v.value().to_le_bytes());
    }

    pub fn finalize(self) -> [u8; 32] {
        self.0.finalize32()
    }
}

impl Default for PeriodicLeafHasher {
    fn default() -> PeriodicLeafHasher {
        PeriodicLeafHasher::new()
    }
}

/// Hash two child digests into their parent, with a distinct domain tag.
pub(super) fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut k = Keccak::new(512, 32, 0x01);
    k.update(DOM_NODE);
    k.update(left);
    k.update(right);
    k.finalize32()
}

#[cfg(test)]
mod kat_tests {
    use super::*;
    extern crate std;

    /// A known answer for the wide periodic leaf and a two leaf root, so a
    /// second implementation can be checked against a number rather than a
    /// description. The leaf is the domain tag followed by each value's
    /// canonical little endian 8 bytes, keccak256 over the whole buffer; the
    /// node is its own tag followed by the two child digests.
    #[test]
    fn the_periodic_leaf_kat() {
        let a = [
            Fp::from_u64(1),
            Fp::from_u64(2),
            Fp::from_u64(3),
            Fp::from_u64(4),
        ];
        let b = [
            Fp::from_u64(0xFFFF_FFFF_0000_0000),
            Fp::from_u64(7),
            Fp::from_u64(0),
            Fp::from_u64(0xFFFF_FFFF_0000_0000),
        ];
        let la = hash_leaf_wide_periodic(&a);
        let lb = hash_leaf_wide_periodic(&b);
        let root = hash_node(&la, &lb);
        let hex = |d: &[u8; 32]| {
            let mut s = std::string::String::new();
            for x in d.iter() {
                s.push_str(&std::format!("{x:02x}"));
            }
            s
        };
        std::println!("PERIODIC_LEAF  [1,2,3,4]                 = {}", hex(&la));
        std::println!("PERIODIC_LEAF  [p-1 as 2^64-2^32, 7,0,.] = {}", hex(&lb));
        std::println!("PERIODIC_ROOT2 node(la, lb)              = {}", hex(&root));
    }
}
