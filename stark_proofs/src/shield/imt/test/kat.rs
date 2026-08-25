// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::imt::hash::{empty_leaf, genesis_root, hasher, hex, leaf_hash, pack};
use crate::shield::imt::Leaf;

fn key(v: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(v);
    k
}

/// spec/nullifier-imt.json, emitted by GenNullifierImt.t.sol from the deployed
/// hash2. Two derivations meeting on one artifact: if these disagree the drift is
/// in a limb order or a padding, and it is found here rather than at settlement.
#[test]
fn the_leaf_matches_the_published_vector() {
    let h = hasher();
    let l = Leaf {
        value: key(1),
        next_index: 2,
        next_value: key(3),
        is_last: false,
    };
    assert_eq!(
        hex(&pack(&leaf_hash(&h, &l))),
        "0x393cba9c22396fe24403248c5e9b4db1179ab7d4b026e4139b5218ce5bb0b658"
    );
}

#[test]
fn the_sentinel_matches_the_published_vector() {
    let h = hasher();
    assert_eq!(
        hex(&pack(&leaf_hash(&h, &Leaf::sentinel()))),
        "0x44fd05a11979d62a9ca870541c8ea2e007831461cac785c7c7f866b4e7e27b0d"
    );
}

#[test]
fn the_empty_leaf_matches_the_published_vector() {
    let h = hasher();
    assert_eq!(
        hex(&pack(&empty_leaf(&h))),
        "0xddf5a3d837317a7a9026cd377e0292a16d71df4c29d0584959b187aa38acc055"
    );
}

#[test]
fn the_genesis_root_matches_the_published_vector() {
    let h = hasher();
    assert_eq!(
        hex(&pack(&genesis_root(&h, 32))),
        "0x5be12be9c1a8dd05f2472735da278b6e67a834bc3fa2e3b2da9d4a7f1e4d4101"
    );
}
