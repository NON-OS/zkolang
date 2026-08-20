// NONOS Operating System (AGPL-3.0-or-later)
//! The serial and parallel provers must emit the same proof, byte for byte.
//!
//! `parallel` is a compile-time cfg, so the two forms can never run in one
//! process and no single test can compare them. What a test can do is emit a
//! digest of a fixed proof; CI runs this under both settings and compares the
//! two lines. That is the gate `par.rs` describes, and until now it described
//! something that did not exist.
//!
//! A divergence here means a parallel map that is not order preserving, or a
//! search that returns a different witness than the serial one. Either produces
//! a proof that still verifies against itself, so nothing else catches it.

use crate::crypto::stark::air::{serialize_proof_ext, stark_prove_ext, stark_verify_ext};
use crate::shield::key::Break;
use crate::shield::test::scenario::balanced;

/// Print the digest of a proof over a fixed witness. Deterministic: the
/// transcript is Fiat-Shamir over the same inputs and the grind returns the
/// lowest nonce, whichever way the crate was built.
#[test]
#[ignore]
fn emit_proof_digest() {
    let js = balanced(Break::None);
    let proof = stark_prove_ext(&js.wired, &js.witness, 32, 8);
    assert!(stark_verify_ext(&js.wired, &proof, 32, 8), "the fixed proof did not verify");
    let bytes = serialize_proof_ext(&proof);

    // A small rolling digest, so the line is short and a single differing byte
    // still moves it. No dependency, and the value only has to be comparable
    // with itself across two builds.
    let mut a: u64 = 0xcbf2_9ce4_8422_2325;
    for b in &bytes {
        a ^= *b as u64;
        a = a.wrapping_mul(0x0000_0100_0000_01b3);
    }
    std::println!("PROOFDIGEST len={} fnv={a:016x}", bytes.len());
}
