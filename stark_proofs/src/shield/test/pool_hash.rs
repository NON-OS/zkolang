// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::hasher;
use crate::crypto::stark::air::{NOTE_LIMBS, RATE};
use crate::crypto::stark::field::Fp;

/// The deployed digest from spec/poseidon-constants.json, which
/// PoseidonGoldilocks.commitNote is gated against. Determinism and binding hold
/// for any self consistent hash, so without this pin a change to the permutation
/// surfaces only downstream, after notes exist. Do not re-baseline.
#[test]
fn the_pool_hash_is_frozen_to_the_deployed_digest() {
    let mut limbs = [Fp::ZERO; NOTE_LIMBS];
    for (i, l) in limbs.iter_mut().enumerate() {
        *l = Fp::from_u64(i as u64 + 1);
    }
    let want: [u64; RATE] = [
        6455909588408588117,
        11340027322162162298,
        9042362242223743603,
        14573159163843564693,
    ];
    let got = hasher().commit_note(&limbs);
    assert_eq!(got.map(|v| v.value()), want);
}
