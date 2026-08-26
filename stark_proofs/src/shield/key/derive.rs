// NONOS Operating System (AGPL-3.0-or-later)

use super::domain::{tag, NULL_DOMAIN, SPEND_DOMAIN};
use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;

pub struct Keys {
    pub spend_pk: [Fp; RATE],
    pub nk: [Fp; RATE],
}

/// Both keys descend from one secret. A free nullifier key would let the same
/// commitment yield a fresh nullifier per key, and would let anyone who has seen
/// a commitment retire a note they do not own.
pub fn derive(h: &Poseidon, sk: [Fp; RATE]) -> Keys {
    Keys { spend_pk: h.compress(&sk, &tag(SPEND_DOMAIN)), nk: h.compress(&sk, &tag(NULL_DOMAIN)) }
}

/// The leaf position is in the preimage. Identical notes commit identically, so
/// without it two deposits share a nullifier and spending one locks the other.
pub fn nullifier(
    h: &Poseidon,
    nk: [Fp; RATE],
    cm: [Fp; RATE],
    leaf_index: u64,
) -> [Fp; RATE] {
    let t = h.compress(&nk, &cm);
    h.compress(&t, &tag(leaf_index))
}
