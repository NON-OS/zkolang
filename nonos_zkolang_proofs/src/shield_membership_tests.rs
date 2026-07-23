/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The shielded-membership program, proven. A secret note commits to a value and a
//! blinding, sits at a leaf of a Merkle tree, and the program proves it reaches a public
//! root without revealing value, blinding, leaf, or path. The honest note proves against
//! the root the same climb computes; a note that does not reach the root has no proof at
//! all. This is a real program in the language, not a fragment: a commitment with a block
//! body, four levels of authentication path, and the bit constraints that hold it up.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_witness};

// Read an example and resolve its includes from stdlib and examples, the way the tool does.
fn program(name: &str) -> String {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.pop();
    let src = fs::read_to_string(root.join("examples").join(name)).expect("read example");
    let mut resolve = |inc: &str| {
        for dir in ["stdlib", "examples"] {
            if let Ok(s) = fs::read_to_string(root.join(dir).join(inc)) {
                return Some(s);
            }
        }
        None
    };
    expand_includes(&src, &mut resolve).expect("expand includes")
}

// value, blinding, then the four siblings and the four direction bits, in declaration order.
const WITNESS: [u64; 10] = [7, 42, 100, 200, 300, 400, 1, 0, 1, 0];

// The root the same witness climbs to, from a variant of the program that outputs the node
// it reaches instead of asserting it against a public root. Keeping the derivation in the
// language keeps it honest: the root is what this exact climb computes, not a second copy.
fn root_for(src: &str, witness: &[u64]) -> u64 {
    let computer = src
        .replace("input root;", "")
        .replace("assert n3 == root;", "output n3;");
    let report = prove_source_with_witness(&computer, &[], witness).expect("climb");
    assert!(report.verified);
    report.outputs[0]
}

#[test]
fn a_note_in_the_set_proves_against_its_root() {
    let src = program("shield_membership.zkl");
    let root = root_for(&src, &WITNESS);
    let report = prove_source_with_witness(&src, &[root], &WITNESS).expect("prove");
    assert!(report.verified, "an honest note was rejected");
}

#[test]
fn a_note_outside_the_set_has_no_proof() {
    // A root the climb does not reach: the final equality has no satisfying trace, so the
    // prover cannot produce a proof, rather than producing one that fails to verify.
    let src = program("shield_membership.zkl");
    let root = root_for(&src, &WITNESS);
    let result = prove_source_with_witness(&src, &[root + 1], &WITNESS);
    let rejected = match result {
        Err(_) => true,
        Ok(report) => !report.verified,
    };
    assert!(rejected, "a note outside the set was accepted");
}

#[test]
fn a_forged_direction_bit_has_no_proof() {
    // The direction bits are asserted to be bits. A path that sets one to two cannot order
    // its level honestly, and the bit constraint has no satisfying trace.
    let src = program("shield_membership.zkl");
    let root = root_for(&src, &WITNESS);
    let mut forged = WITNESS;
    forged[6] = 2; // dir0 is no longer a bit
    let result = prove_source_with_witness(&src, &[root], &forged);
    let rejected = match result {
        Err(_) => true,
        Ok(report) => !report.verified,
    };
    assert!(rejected, "a non-bit direction was accepted");
}
