/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! MiMC as a zKolang program, proven equal to the field reference. The same module
//! emits the source and computes the reference, so this checks the language really
//! computes the hash: compile the emitted program, run it on an input, prove the
//! trace, and the public output must equal the reference value for that input.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_inputs};

use crate::mimc;

// Resolve a stdlib include from disk, so a test program can call a library gadget.
fn stdlib_resolve(name: &str) -> Option<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    fs::read_to_string(base.join("stdlib").join(name)).ok()
}

#[test]
fn the_permute_gadget_matches_the_mimc_reference() {
    // The stdlib `permute` gadget must equal the independent field reference, so every
    // program that calls it hashes with the exact MiMC permutation the hand loops did.
    // Nothing else pins an absolute digest, so this is what keeps the primitive honest.
    let src = "include \"hash.zkl\";\ninput x;\noutput permute(x);\n";
    let mut resolve = stdlib_resolve;
    let expanded = expand_includes(src, &mut resolve).expect("expand");
    for input in [0u64, 1, 2, 7, 42, 1000, u32::MAX as u64] {
        let report = prove_source_with_inputs(&expanded, &[input]).expect("run");
        assert!(report.verified, "permute did not verify for {input}");
        assert_eq!(
            report.outputs,
            vec![mimc::reference(input)],
            "permute disagreed with the MiMC reference for input {input}"
        );
    }
}

#[test]
fn mimc_in_zkolang_matches_the_field_reference() {
    let src = mimc::source();
    for input in [0u64, 1, 2, 7, 42, 1000, u32::MAX as u64] {
        let report = prove_source_with_inputs(&src, &[input]).expect("run");
        assert!(report.verified, "the MiMC trace did not verify for {input}");
        assert_eq!(
            report.outputs,
            vec![mimc::reference(input)],
            "zKolang MiMC disagreed with the reference for input {input}"
        );
    }
}

#[test]
fn mimc_is_a_deterministic_function() {
    let src = mimc::source();
    let a = prove_source_with_inputs(&src, &[12345]).expect("run");
    let b = prove_source_with_inputs(&src, &[12345]).expect("run");
    assert_eq!(
        a.outputs, b.outputs,
        "the same input gave two different digests"
    );
}

#[test]
fn mimc_separates_distinct_inputs() {
    // A hash should not collapse two nearby inputs to the same digest.
    let src = mimc::source();
    let a = prove_source_with_inputs(&src, &[1]).expect("run");
    let b = prove_source_with_inputs(&src, &[2]).expect("run");
    assert_ne!(a.outputs, b.outputs, "two inputs collided");
}
