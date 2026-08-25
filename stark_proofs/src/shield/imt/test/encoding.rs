// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{NOTE_DOMAIN, NOTE_LIMBS, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::imt::{Leaf, IMT_LEAF_DOMAIN, IMT_LEAF_LIMBS};

/// The tag sits at the first limb past the payload, and no two structures share
/// both a payload length and a tag. Without that rule the separation would hold
/// only because today's fields happen not to equal a tag.
#[test]
fn the_leaf_cannot_alias_a_note_commitment() {
    assert_ne!(
        (IMT_LEAF_LIMBS, IMT_LEAF_DOMAIN),
        (NOTE_LIMBS, NOTE_DOMAIN),
        "two structures share a payload length and a tag, so their hashes alias"
    );
}

#[test]
fn the_tag_follows_the_payload_and_the_rest_is_zero() {
    let l = Leaf::sentinel().limbs();
    assert_eq!(l[IMT_LEAF_LIMBS], Fp::from_u64(IMT_LEAF_DOMAIN));
    assert!(l[IMT_LEAF_LIMBS + 1..].iter().all(|v| *v == Fp::ZERO));
}

/// The empty set: below every key, pointing nowhere, and the flag says so rather
/// than a value pretending to be the maximum.
#[test]
fn the_sentinel_is_last_and_points_nowhere() {
    let s = Leaf::sentinel();
    assert!(s.is_last);
    assert_eq!(s.value, [Fp::ZERO; RATE]);
    assert_eq!(s.next_value, [Fp::ZERO; RATE]);
}

/// Changing any field moves the limbs, so the encoding does not collapse two
/// leaves onto one commitment.
#[test]
fn every_field_moves_the_encoding() {
    let base = Leaf::sentinel();
    let mut seen = alloc::vec![base.limbs()];
    let mut v = base;
    v.value[0] = Fp::from_u64(7);
    seen.push(v.limbs());
    let mut n = base;
    n.next_index = 3;
    seen.push(n.limbs());
    let mut w = base;
    w.next_value[2] = Fp::from_u64(9);
    seen.push(w.limbs());
    let mut f = base;
    f.is_last = false;
    seen.push(f.limbs());
    let before = seen.len();
    seen.sort_unstable_by(|a, b| a.iter().map(|x| x.value()).cmp(b.iter().map(|x| x.value())));
    seen.dedup();
    assert_eq!(seen.len(), before, "two different leaves share an encoding");
}
