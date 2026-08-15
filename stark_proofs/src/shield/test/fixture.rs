// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::key::derive;
use crate::shield::note::{Note, POOL_LOG_ROUNDS};

pub(super) fn hasher() -> Poseidon {
    Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
}

pub(super) fn secret(seed: u64) -> [Fp; RATE] {
    let mut sk = [Fp::ZERO; RATE];
    for (i, v) in sk.iter_mut().enumerate() {
        *v = Fp::from_u64(seed * 16 + i as u64 + 1);
    }
    sk
}

/// A note the holder of `sk` can spend: its committed spend key is the one that
/// secret derives. Built any other way the note is unspendable.
pub(super) fn owned(sk: [Fp; RATE], seed: u64, value: u64) -> Note {
    let k = derive(&hasher(), sk);
    Note {
        value,
        asset_id: 0,
        spend_pk: [
            k.spend_pk[0].value(),
            k.spend_pk[1].value(),
            k.spend_pk[2].value(),
            k.spend_pk[3].value(),
        ],
        blinding: [seed + 5, seed + 6, seed + 7, seed + 8],
    }
}

pub(super) fn plain(seed: u64, value: u64) -> Note {
    Note {
        value,
        asset_id: 0,
        spend_pk: [seed + 1, seed + 2, seed + 3, seed + 4],
        blinding: [seed + 5, seed + 6, seed + 7, seed + 8],
    }
}
