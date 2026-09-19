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

use crate::crypto::stark::air::{
    periodic_root, serialize_proof_ext, stark_prove_ext, stark_prove_ext_preprocessed,
    stark_verify_ext, stark_verify_ext_preprocessed,
};
use crate::proof_wire::serialize_pre;
use crate::shield::key::Break;
use crate::shield::test::scenario::balanced;

/// The rolling digest the two lines below print. Small, so a line is short,
/// and a single differing byte still moves it. No dependency, and the value
/// only has to be comparable with itself across two builds.
fn fnv(bytes: &[u8]) -> u64 {
    let mut a: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        a ^= *b as u64;
        a = a.wrapping_mul(0x0000_0100_0000_01b3);
    }
    a
}

/// Print the digest of a proof over a fixed witness. Deterministic: the
/// transcript is Fiat-Shamir over the same inputs and the grind returns the
/// lowest nonce, whichever way the crate was built.
#[test]
#[ignore]
fn emit_proof_digest() {
    let js = balanced(Break::None);
    let proof = stark_prove_ext(&js.wired, &js.witness, 32, 8);
    assert!(
        stark_verify_ext(&js.wired, &proof, 32, 8),
        "the fixed proof did not verify"
    );
    let bytes = serialize_proof_ext(&proof);
    std::println!("PROOFDIGEST len={} fnv={:016x}", bytes.len(), fnv(&bytes));
}

/// The same gate for the preprocessed prover, which is the settlement path,
/// at two rates. The composition and DEEP kernels are the parts of that prover
/// that get rewritten for speed, and a rewrite that is right produces the same
/// bytes: every value it computes is a field element the old kernel computed
/// too, in a different order. A line that moves here is a kernel that
/// computes something else.
#[test]
#[ignore]
fn emit_pre_proof_digest() {
    let js = balanced(Break::None);
    for extra in [0u32, 1] {
        let pre = stark_prove_ext_preprocessed(&js.wired, &js.witness, 32, 8, extra)
            .expect("nothing watches this proof, so nothing can cancel it");
        let root = periodic_root(&js.wired, extra);
        assert!(
            stark_verify_ext_preprocessed(&js.wired, &pre, 32, 8, extra, &root),
            "the fixed preprocessed proof did not verify at extra blowup {extra}"
        );
        let bytes = serialize_pre(&pre);
        std::println!(
            "PREPROOFDIGEST extra={extra} len={} fnv={:016x}",
            bytes.len(),
            fnv(&bytes)
        );
    }
}
