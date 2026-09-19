// NONOS Operating System (AGPL-3.0-or-later)
//! The live Sepolia pool's tree, rebuilt here and checked against its root.
//!
//! Before any box time goes into proving a spend against a real pool, one
//! question has to be answered and it is not about the spend: does this
//! circuit's tree agree with the deployed contract's? Both insert the same
//! commitments at the same depth under the same hash, and if the zero
//! subtree table, the hash parameters or the limb packing differ anywhere,
//! both sides still produce a perfectly well formed root and they are not
//! the same root. A membership proof built against ours would then walk to
//! a value the pool has never held, and the statement would be unprovable
//! rather than wrong, which is the expensive failure.
//!
//! The commitments below were read from the chain's own `NoteCommitted`
//! logs, not copied from the handoff note, so this also confirms the two
//! notes we were given are the leaves the pool actually stored.

use super::depth::DEPLOYED;
use super::fixture::hasher;
use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::member::PoolTree;

/// A bytes32 digest as the four limbs the circuit uses: big endian over
/// `[limb3, limb2, limb1, limb0]`, so limb 0 is the last eight bytes.
fn limbs(hex: &str) -> [Fp; RATE] {
    let b: alloc::vec::Vec<u8> = (0..32)
        .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
        .collect();
    let mut out = [Fp::ZERO; RATE];
    for (j, slot) in out.iter_mut().enumerate() {
        let hi = 24 - 8 * j;
        let mut w = [0u8; 8];
        w.copy_from_slice(&b[hi..hi + 8]);
        *slot = Fp::from_u64(u64::from_be_bytes(w));
    }
    out
}

fn hex_of(d: [Fp; RATE]) -> alloc::string::String {
    let mut s = alloc::string::String::new();
    for j in (0..RATE).rev() {
        s.push_str(&alloc::format!("{:016x}", d[j].to_u64()));
    }
    s
}

/// Pool 0xa760D749adfe15BafFfeC8E7301CEA89B150c046 on Sepolia, three leaves,
/// read from its `NoteCommitted` logs at blocks 11,733,781 / 834 / 900.
const LEAF0: &str = "0f14542ea9ec2683cac6fb8e8167203c8c7eed6759da835fd29c233259e8ba62";
const LEAF1: &str = "59b6835c74c863e7b9defa6bc7c34e1ad9ee1c8519a6fe0f301a2502cd927fd8";
const LEAF2: &str = "1edff88b113db999adadd2406035b75c13c3eb69ee244dc4f56ecaf36e544897";

/// The root the pool reports after leaf 2, and the one the spend is proved
/// against.
const ROOT: &str = "e083f588523bb322c7d4077ecf9d36f1628a2655c22c86136c73b6b3da159a3f";

#[test]
fn the_circuit_rebuilds_the_live_pool_root() {
    let mut tree = PoolTree::with_depth(hasher(), DEPLOYED);
    for leaf in [LEAF0, LEAF1, LEAF2] {
        tree.insert(limbs(leaf));
    }
    let got = hex_of(tree.root());
    assert_eq!(
        got, ROOT,
        "the circuit's tree disagrees with the deployed pool; a membership \
         proof built here would walk to a root the pool has never held"
    );
}

/// The openings the spend needs, and the property that makes them openings:
/// walking each leaf's siblings must arrive at the same root. Printed as
/// well, under `--nocapture`, because these are what the proof consumes.
#[test]
fn both_notes_open_to_the_live_root() {
    let h = hasher();
    let mut tree = PoolTree::with_depth(h.clone(), DEPLOYED);
    for leaf in [LEAF0, LEAF1, LEAF2] {
        tree.insert(limbs(leaf));
    }
    let root = tree.root();

    for (index, leaf) in [(1usize, LEAF1), (2usize, LEAF2)] {
        let (sibs, dirs) = tree.path(index);
        assert_eq!(
            sibs.len(),
            DEPLOYED,
            "an opening must be one sibling per level"
        );
        assert_eq!(dirs.len(), DEPLOYED);

        let mut node = limbs(leaf);
        for (sib, right) in sibs.iter().zip(dirs.iter()) {
            node = if *right {
                h.compress(sib, &node)
            } else {
                h.compress(&node, sib)
            };
        }
        assert_eq!(
            node, root,
            "leaf {index} does not open to the live root under this circuit's tree"
        );
        std::println!("leaf {index} opens to {}", hex_of(root));
    }
}
