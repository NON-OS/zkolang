// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::hasher;
use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::key::{derive, nullifier};
use crate::shield::note::{note_parts, Note};

fn secret(seed: u64) -> [Fp; RATE] {
    let mut sk = [Fp::ZERO; RATE];
    for (i, v) in sk.iter_mut().enumerate() {
        *v = Fp::from_u64(seed * 16 + i as u64 + 1);
    }
    sk
}

/// The circuit derives what spec/shield-key-hierarchy.json publishes. That file
/// is what the wallet and the client implement against, so a disagreement here
/// means notes the pool accepts and the holder cannot spend.
#[test]
fn the_circuit_derives_the_published_vector() {
    let raw = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../spec/shield-key-hierarchy.json"
    ))
    .expect("key hierarchy vector missing");

    let h = hasher();
    let sk = secret(1);
    let k = derive(&h, sk);
    let note = Note {
        value: 1_000,
        asset_id: 0,
        spend_pk: k.spend_pk.map(|v| v.value()),
        blinding: [6, 7, 8, 9],
    };
    let cm = note_parts(&note).cm;
    let nf = nullifier(&h, k.nk, cm, 0);

    for d in [k.spend_pk, k.nk, cm, nf] {
        let s: alloc::vec::Vec<alloc::string::String> =
            d.iter().map(|v| alloc::format!("{}", v.value())).collect();
        let joined = alloc::format!("[{}]", s.join(","));
        assert!(raw.contains(&joined), "vector is missing {joined}");
    }
}
