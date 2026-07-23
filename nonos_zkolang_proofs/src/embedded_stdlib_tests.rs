/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The kernel path: no filesystem, so includes resolve from the standard library baked into the
//! crate. A program that includes a standard gadget compiles and proves with no file access, the
//! way the kernel terminal and editor run it; an include the standard library does not name is
//! reported, not silently dropped.

use nonos_zkolang::{expand_with_stdlib, prove_source_with_inputs, stdlib_source};

#[test]
fn a_program_proves_against_the_embedded_stdlib() {
    // Include a gadget, use it, and prove it, all with no file access.
    let src = "include \"math.zkl\";\ninput x;\noutput cube(x);";
    let expanded = expand_with_stdlib(src).expect("expand");
    let report = prove_source_with_inputs(&expanded, &[4]).expect("run");
    assert!(report.verified, "an embedded-stdlib program was rejected");
    assert_eq!(report.outputs, vec![64], "cube of four");
}

#[test]
fn the_whole_standard_library_is_embedded() {
    // Every module resolves from the binary and expands, transitive includes included, so a
    // program on the kernel can include any of them without a filesystem.
    let modules = [
        "bits.zkl",
        "cmp.zkl",
        "curve.zkl",
        "encode.zkl",
        "field.zkl",
        "gate.zkl",
        "hash.zkl",
        "logic.zkl",
        "math.zkl",
        "merkle.zkl",
        "order.zkl",
        "poly.zkl",
        "select.zkl",
        "vm.zkl",
    ];
    for name in modules {
        assert!(stdlib_source(name).is_some(), "{name} is not embedded");
        let src = format!("include \"{name}\";\ninput x;\noutput x;");
        expand_with_stdlib(&src).unwrap_or_else(|e| panic!("{name} did not expand: {e:?}"));
    }
    // An include the standard library does not name is reported, not dropped.
    assert!(
        expand_with_stdlib("include \"nope.zkl\";").is_err(),
        "an unknown include was not reported"
    );
}
