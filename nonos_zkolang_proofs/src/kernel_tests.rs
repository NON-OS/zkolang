/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The kernel circuits, every one proven. These are the jobs the NONOS kernel expresses
//! in zKolang at its trust boundary: attesting a capsule by its measurement, enforcing
//! anti-rollback against the TPM floor, checking a capability by set membership, folding
//! a boot measurement chain, sealing data to a measurement, and authorizing a syscall by
//! its capability token. Each has an accept case and, where it enforces a rule, a reject
//! case with no proof.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_inputs, prove_source_with_witness};

fn root() -> PathBuf {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    base
}

fn resolve(path: &str) -> Option<String> {
    fs::read_to_string(root().join("stdlib").join(path)).ok()
}

fn program(rel: &str) -> String {
    let src = fs::read_to_string(root().join(rel)).expect("read");
    expand_includes(&src, &mut resolve).expect("expand")
}

fn proves_w(rel: &str, public: &[u64], witness: &[u64]) -> bool {
    prove_source_with_witness(&program(rel), public, witness)
        .map(|r| r.verified)
        .unwrap_or(false)
}

// Hash helpers that compute an expected value with the same MiMC the circuits use, so a
// test never re-implements the permutation; the source is compiled and run.
fn mimc_inline(src: &str, inputs: &[u64]) -> u64 {
    let expanded = expand_includes(src, &mut resolve).expect("expand");
    prove_source_with_inputs(&expanded, inputs)
        .expect("run")
        .outputs[0]
}

#[test]
fn attestation_binds_content_to_a_measurement() {
    let expected = prove_source_with_inputs(&program("examples/sponge2.zkl"), &[7, 9])
        .expect("hash")
        .outputs[0];
    assert!(proves_w("circuits/kernel/attest.zkl", &[expected], &[7, 9]));
    assert!(!proves_w(
        "circuits/kernel/attest.zkl",
        &[expected],
        &[7, 10]
    ));
}

#[test]
fn anti_rollback_admits_forward_and_rejects_stale() {
    assert!(proves_w(
        "circuits/kernel/anti_rollback.zkl",
        &[300, 1000],
        &[]
    ));
    assert!(proves_w(
        "circuits/kernel/anti_rollback.zkl",
        &[300, 300],
        &[]
    ));
    assert!(!proves_w(
        "circuits/kernel/anti_rollback.zkl",
        &[1000, 300],
        &[]
    ));
}

#[test]
fn capability_requires_membership_in_the_grant_set() {
    assert!(proves_w(
        "circuits/kernel/capability.zkl",
        &[5, 3, 5, 7, 9],
        &[]
    ));
    assert!(!proves_w(
        "circuits/kernel/capability.zkl",
        &[6, 3, 5, 7, 9],
        &[]
    ));
}

#[test]
fn boot_measurement_root_is_deterministic_and_order_sensitive() {
    let a = prove_source_with_inputs(&program("circuits/kernel/measure_root.zkl"), &[1, 2, 3, 4])
        .expect("run");
    let b = prove_source_with_inputs(&program("circuits/kernel/measure_root.zkl"), &[1, 2, 3, 4])
        .expect("run");
    assert!(a.verified);
    assert_eq!(a.outputs, b.outputs);
    let c = prove_source_with_inputs(&program("circuits/kernel/measure_root.zkl"), &[1, 2, 3, 5])
        .expect("run");
    assert_ne!(a.outputs, c.outputs);
}

#[test]
fn boot_chain8_folds_eight_stages() {
    let a = prove_source_with_inputs(
        &program("circuits/kernel/boot_chain8.zkl"),
        &[1, 2, 3, 4, 5, 6, 7, 8],
    )
    .expect("run");
    assert!(a.verified);
    let b = prove_source_with_inputs(
        &program("circuits/kernel/boot_chain8.zkl"),
        &[1, 2, 3, 4, 5, 6, 7, 9],
    )
    .expect("run");
    assert_ne!(
        a.outputs, b.outputs,
        "a changed final stage did not change the root"
    );
}

#[test]
fn sealing_binds_data_to_a_measurement() {
    let sealed = mimc_inline(
        "include \"hash.zkl\";\n\
         public m;\n public d;\n let s = m + d;\n\
         for i in 0 .. 16 { let s = mimc_round(s, MIMC_C[i]); }\n reveal s;",
        &[555, 42],
    );
    // Correct data under the measurement unseals; wrong data has no proof.
    assert!(proves_w("circuits/kernel/seal.zkl", &[555, sealed], &[42]));
    assert!(!proves_w("circuits/kernel/seal.zkl", &[555, sealed], &[43]));
}

#[test]
fn syscall_authorization_requires_the_capability_token() {
    let token = mimc_inline(
        "include \"hash.zkl\";\n\
         public bk;\n public cid;\n public sc;\n let t = bk + cid;\n\
         for i in 0 .. 8 { let t = mimc_round(t, MIMC_C[i]); }\n let t = t + sc;\n\
         for i in 8 .. 16 { let t = mimc_round(t, MIMC_C[i]); }\n reveal t;",
        &[9, 100, 5],
    );
    // The genuine token authorizes; a forged token does not.
    assert!(proves_w(
        "circuits/kernel/syscall_auth.zkl",
        &[100, 5, token],
        &[9]
    ));
    assert!(!proves_w(
        "circuits/kernel/syscall_auth.zkl",
        &[100, 5, token + 1],
        &[9]
    ));
}
