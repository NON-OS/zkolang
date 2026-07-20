/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Includes and the standard library. A program pulls gadgets from stdlib files
//! through an include, the resolver reads them from disk, and the expanded source is
//! compiled and proven. Transitive includes and single-inclusion are checked, so a
//! library that depends on another does not redefine it.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_inputs, CompileError};

use crate::mimc;

// Resolve an include by reading it from the repository's stdlib or examples.
fn resolve(path: &str) -> Option<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    for dir in ["stdlib", "examples"] {
        if let Ok(s) = fs::read_to_string(base.join(dir).join(path)) {
            return Some(s);
        }
    }
    None
}

fn example(name: &str) -> String {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    fs::read_to_string(base.join("examples").join(name)).expect("read example")
}

#[test]
fn a_program_includes_the_standard_library() {
    let src = example("uses_stdlib.zkl");
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    let report = prove_source_with_inputs(&expanded, &[2]).expect("run");
    assert!(report.verified);
    assert_eq!(
        report.outputs,
        vec![140],
        "pow7(2) + sq(2) + cube(2) = 128 + 4 + 8"
    );
}

#[test]
fn includes_are_transitive_and_included_once() {
    // mimc_lib includes hash.zkl, which includes math.zkl. The library MiMC must
    // equal the reference, which also proves math.zkl was spliced once, not twice.
    let src = example("mimc_lib.zkl");
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    let report = prove_source_with_inputs(&expanded, &[123]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![mimc::reference(123)]);
}

#[test]
fn a_missing_include_is_an_error() {
    let mut none = |_: &str| None;
    let r = expand_includes("include \"nope.zkl\";", &mut none);
    assert!(matches!(r, Err(CompileError::IncludeNotFound)));
}
