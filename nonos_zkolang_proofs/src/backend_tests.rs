/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The native back-end, checked against the proven trace. For each program, emit C,
//! compile it with the system C compiler, run it on some inputs, and require the
//! native output to equal what the VM produced and the STARK proved. That the same
//! source runs as native code and as a proven trace, and agrees, is the point of a
//! target-independent op list. The Python back-end is emitted and shape-checked here
//! too; it is exercised end to end wherever a Python interpreter is available.

use std::process::Command;

use nonos_zkolang::{compile_source, prove_source_with_inputs, to_asm, to_c, to_python};

// Emit a program as x86_64 assembly to a .S file, assemble and link it with the C
// runtime, run it with the given inputs, and parse the field outputs it prints.
fn run_asm(src: &str, inputs: &[u64], tag: &str) -> Vec<u64> {
    let program = compile_source(src).expect("compile");
    let asm = to_asm(&program);

    let dir = std::env::temp_dir();
    let spath = dir.join(format!("zkolang_{tag}.S"));
    let bpath = dir.join(format!("zkolang_{tag}_asm.bin"));
    std::fs::write(&spath, &asm).expect("write asm");

    let status = Command::new("cc")
        .args([spath.to_str().unwrap(), "-o", bpath.to_str().unwrap()])
        .status()
        .expect("cc");
    assert!(status.success(), "assemble failed for {tag}");

    let mut cmd = Command::new(&bpath);
    for v in inputs {
        cmd.arg(v.to_string());
    }
    let out = cmd.output().expect("run asm");
    assert!(out.status.success(), "asm run failed for {tag}");
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect()
}

// Compile emitted C to a temporary binary, run it with the given inputs, and parse
// the space-separated field outputs it prints.
fn run_native(src: &str, inputs: &[u64], tag: &str) -> Vec<u64> {
    let program = compile_source(src).expect("compile");
    let c = to_c(&program);

    let dir = std::env::temp_dir();
    let cpath = dir.join(format!("zkolang_{tag}.c"));
    let bpath = dir.join(format!("zkolang_{tag}.bin"));
    std::fs::write(&cpath, &c).expect("write c");

    let status = Command::new("cc")
        .arg(&cpath)
        .arg("-O2")
        .arg("-o")
        .arg(&bpath)
        .status()
        .expect("cc");
    assert!(status.success(), "cc failed for {tag}");

    let mut cmd = Command::new(&bpath);
    for v in inputs {
        cmd.arg(v.to_string());
    }
    let out = cmd.output().expect("run native");
    assert!(out.status.success(), "native run failed for {tag}");
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect()
}

// The native emission of a program must produce exactly what the VM proves.
fn agrees(src: &str, inputs: &[u64], tag: &str) {
    let proven = prove_source_with_inputs(src, inputs).expect("run");
    assert!(proven.verified);
    let native = run_native(src, inputs, tag);
    assert_eq!(
        native, proven.outputs,
        "native and proven disagreed for {tag}"
    );
}

#[test]
fn native_c_matches_the_proof_on_arithmetic() {
    agrees("input x; let y = x * x * x; output y;", &[9], "cube");
    agrees("input a; input b; output a * b + a - b;", &[6, 7], "poly");
}

#[test]
fn native_c_matches_the_proof_on_a_table_and_loop() {
    let src = "const C = [1, 2, 3]; input x; let acc = 0;
               for i in 0 .. 3 { let acc = acc * x + C[2 - i]; } output acc;";
    agrees(src, &[5], "horner");
}

#[test]
fn native_c_matches_the_proof_on_a_field_wrapping_hash() {
    // MiMC wraps mod p in every round, so agreement here checks the field reduction
    // in C matches the field the STARK proves over.
    let src = crate::mimc::source();
    agrees(&src, &[123_456_789], "mimc");
}

// The x86_64 emission of a program must produce exactly what the VM proves. This is
// the same target-independence check as the C target, one field lower down: hand
// written Goldilocks arithmetic in assembly agreeing with the proven trace.
fn asm_agrees(src: &str, inputs: &[u64], tag: &str) {
    let proven = prove_source_with_inputs(src, inputs).expect("run");
    assert!(proven.verified);
    let native = run_asm(src, inputs, tag);
    assert_eq!(native, proven.outputs, "asm and proven disagreed for {tag}");
}

#[test]
#[cfg(target_arch = "x86_64")]
fn native_asm_matches_the_proof_on_arithmetic() {
    asm_agrees("input x; let y = x * x * x; output y;", &[9], "cube");
    asm_agrees("input a; input b; output a * b + a - b;", &[6, 7], "poly");
}

#[test]
#[cfg(target_arch = "x86_64")]
fn native_asm_matches_the_proof_on_a_table_loop_and_inverse() {
    let horner = "const C = [1, 2, 3]; input x; let acc = 0;
                  for i in 0 .. 3 { let acc = acc * x + C[2 - i]; } output acc;";
    asm_agrees(horner, &[5], "horner");
    // inv exercises the Fermat exponentiation path in the assembly prelude.
    asm_agrees("input x; output inv(x) * x;", &[7], "inverse");
}

#[test]
#[cfg(target_arch = "x86_64")]
fn native_asm_matches_the_proof_on_a_field_wrapping_hash() {
    let src = crate::mimc::source();
    asm_agrees(&src, &[123_456_789], "mimc");
}

#[test]
fn the_python_backend_emits_a_runnable_module() {
    // Shape check: the emitted Python defines run(inputs) and the field prelude. It
    // is executed end to end where an interpreter is present.
    let program = compile_source("input x; let y = x * x; output y;").expect("compile");
    let py = to_python(&program);
    assert!(py.contains("def run(inputs):"));
    assert!(py.contains("P = 0xFFFFFFFF00000001"));
    assert!(py.contains("_mul(r["));
}
