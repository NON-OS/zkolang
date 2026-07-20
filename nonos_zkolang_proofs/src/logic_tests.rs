/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The boolean and comparison standard library, proven. Each gadget is included from
//! stdlib, applied to its inputs, and its whole truth table checked by proving the
//! program on every input combination. The gadgets are written in zKolang, so this
//! proves the language expresses boolean logic and equality, not just arithmetic.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_inputs, prove_source_with_witness};

fn resolve(path: &str) -> Option<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    fs::read_to_string(base.join("stdlib").join(path)).ok()
}

// Prove `output <gadget>(a, b);` on the given inputs and return the single output.
fn eval2(lib: &str, gadget: &str, a: u64, b: u64) -> u64 {
    let src = format!("include \"{lib}\";\ninput x;\ninput y;\noutput {gadget}(x, y);");
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    let report = prove_source_with_inputs(&expanded, &[a, b]).expect("run");
    assert!(report.verified);
    report.outputs[0]
}

fn eval1(lib: &str, gadget: &str, a: u64) -> u64 {
    let src = format!("include \"{lib}\";\ninput x;\noutput {gadget}(x);");
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    let report = prove_source_with_inputs(&expanded, &[a]).expect("run");
    assert!(report.verified);
    report.outputs[0]
}

#[test]
fn not_gate() {
    assert_eq!(eval1("logic.zkl", "not", 0), 1);
    assert_eq!(eval1("logic.zkl", "not", 1), 0);
}

#[test]
fn and_gate() {
    let t = |a, b| eval2("logic.zkl", "and", a, b);
    assert_eq!([t(0, 0), t(0, 1), t(1, 0), t(1, 1)], [0, 0, 0, 1]);
}

#[test]
fn or_gate() {
    let t = |a, b| eval2("logic.zkl", "or", a, b);
    assert_eq!([t(0, 0), t(0, 1), t(1, 0), t(1, 1)], [0, 1, 1, 1]);
}

#[test]
fn xor_gate() {
    let t = |a, b| eval2("logic.zkl", "xor", a, b);
    assert_eq!([t(0, 0), t(0, 1), t(1, 0), t(1, 1)], [0, 1, 1, 0]);
}

#[test]
fn nand_and_nor_and_implies() {
    assert_eq!(
        [
            eval2("logic.zkl", "nand", 1, 1),
            eval2("logic.zkl", "nand", 1, 0)
        ],
        [0, 1]
    );
    assert_eq!(
        [
            eval2("logic.zkl", "nor", 0, 0),
            eval2("logic.zkl", "nor", 1, 0)
        ],
        [1, 0]
    );
    assert_eq!(
        [
            eval2("logic.zkl", "implies", 1, 0),
            eval2("logic.zkl", "implies", 1, 1)
        ],
        [0, 1]
    );
}

// The 4-bit less-than proof: the difference d = b - a - 1 is exhibited in bits, and
// the program is `include "logic.zkl"; input a; input b; secret d0..d3; ...`.
fn less_than(a: u64, b: u64) -> bool {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    let src = fs::read_to_string(base.join("examples").join("less_than.zkl")).unwrap();
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    // Witness: the four bits of (b - a - 1), or garbage when a >= b (no valid witness).
    let diff = b.wrapping_sub(a).wrapping_sub(1);
    let bits = [diff & 1, (diff >> 1) & 1, (diff >> 2) & 1, (diff >> 3) & 1];
    prove_source_with_witness(&expanded, &[a, b], &bits)
        .map(|r| r.verified)
        .unwrap_or(false)
}

#[test]
fn ordered_comparison_accepts_less_and_rejects_not_less() {
    assert!(less_than(3, 10), "3 < 10 should prove");
    assert!(less_than(0, 1), "0 < 1 should prove");
    assert!(!less_than(10, 3), "10 < 3 has no proof");
    assert!(!less_than(5, 5), "5 < 5 has no proof");
}

#[test]
fn equality_and_zero_tests() {
    assert_eq!(eval2("cmp.zkl", "is_equal", 7, 7), 1);
    assert_eq!(eval2("cmp.zkl", "is_equal", 7, 8), 0);
    assert_eq!(eval1("cmp.zkl", "is_zero", 0), 1);
    assert_eq!(eval1("cmp.zkl", "is_zero", 5), 0);
    assert_eq!(eval2("cmp.zkl", "is_distinct", 3, 4), 1);
}
