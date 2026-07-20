/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The standard library gadgets, proven by known answers. Each module is included and
//! its gadgets applied to fixed inputs, and the proven output is checked, so the library
//! is real, reusable zKolang, not illustration. The polynomial and selection gadgets are
//! cheap; the ordering gadgets carry a comparison and are checked on a few points.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_inputs};

fn resolve(path: &str) -> Option<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    fs::read_to_string(base.join("stdlib").join(path)).ok()
}

fn out(src: &str, inputs: &[u64]) -> u64 {
    let expanded = expand_includes(src, &mut resolve).expect("expand");
    prove_source_with_inputs(&expanded, inputs)
        .expect("run")
        .outputs[0]
}

#[test]
fn polynomial_gadgets() {
    assert_eq!(
        out(
            "include \"poly.zkl\";\npublic x;\nreveal line(2, 3, x);",
            &[5]
        ),
        13
    );
    assert_eq!(
        out(
            "include \"poly.zkl\";\npublic x;\nreveal quad(1, 2, 3, x);",
            &[2]
        ),
        11
    );
    assert_eq!(
        out(
            "include \"poly.zkl\";\npublic x;\nreveal cubic(2, 0, 0, 1, x);",
            &[3]
        ),
        55
    );
    assert_eq!(
        out(
            "include \"poly.zkl\";\npublic t;\nreveal lerp(10, 20, t);",
            &[1]
        ),
        20
    );
    assert_eq!(
        out(
            "include \"poly.zkl\";\npublic t;\nreveal lerp(10, 20, t);",
            &[0]
        ),
        10
    );
}

#[test]
fn selection_gadgets() {
    let mux4 = |s1, s0| {
        let src =
            "include \"select.zkl\";\npublic s1;\npublic s0;\nreveal mux4(s1, s0, 10, 20, 30, 40);";
        out(src, &[s1, s0])
    };
    assert_eq!(
        [mux4(0, 0), mux4(0, 1), mux4(1, 0), mux4(1, 1)],
        [10, 20, 30, 40]
    );
    assert_eq!(
        out(
            "include \"select.zkl\";\npublic c;\nreveal cond(c, 7, 9);",
            &[1]
        ),
        7
    );
    assert_eq!(
        out(
            "include \"select.zkl\";\npublic c;\nreveal cond(c, 7, 9);",
            &[0]
        ),
        9
    );
}

#[test]
fn ordering_gadgets() {
    assert_eq!(
        out(
            "include \"order.zkl\";\npublic a;\npublic b;\nreveal min(a, b);",
            &[3, 10]
        ),
        3
    );
    assert_eq!(
        out(
            "include \"order.zkl\";\npublic a;\npublic b;\nreveal max(a, b);",
            &[3, 10]
        ),
        10
    );
    assert_eq!(
        out(
            "include \"order.zkl\";\npublic x;\nreveal clamp(x, 10, 100);",
            &[5]
        ),
        10
    );
    assert_eq!(
        out(
            "include \"order.zkl\";\npublic x;\nreveal clamp(x, 10, 100);",
            &[50]
        ),
        50
    );
}

#[test]
fn the_alu_selects_an_operation_by_a_match() {
    // op 0 adds, 1 subtracts, 2 multiplies, anything else takes the minimum.
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    let file = fs::read_to_string(base.join("examples/alu.zkl")).expect("read");
    let src = expand_includes(&file, &mut resolve).expect("expand");
    let run = |op, a, b| {
        prove_source_with_inputs(&src, &[op, a, b])
            .expect("run")
            .outputs[0]
    };
    assert_eq!(run(0, 8, 5), 13);
    assert_eq!(run(1, 8, 5), 3);
    assert_eq!(run(2, 8, 5), 40);
    assert_eq!(run(9, 8, 5), 5);
}
