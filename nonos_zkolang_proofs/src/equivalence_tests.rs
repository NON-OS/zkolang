/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The optimizer is behavior-preserving, checked differentially. Each program is compiled
//! twice, with the optimizer and without, and run on the same inputs; the outputs must be
//! identical. This is the guard that would have caught a fold that changed a result, and
//! it runs over the arithmetic, sequence, array, function, and hash programs, the classes
//! an optimizer bug hides in. It runs the VM without proving, so it is fast and covers many
//! programs; ordered-comparison programs are separate, since their advice is filled by the
//! driver, not the raw VM.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate, expand_includes};

fn resolve(path: &str) -> Option<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    fs::read_to_string(base.join("stdlib").join(path)).ok()
}

fn program(rel: &str) -> String {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    let src = fs::read_to_string(base.join("examples").join(rel)).expect("read");
    expand_includes(&src, &mut resolve).expect("expand")
}

// Compile with and without the optimizer, run both, and require identical outputs.
fn same(rel: &str, public: &[u64], secret: &[u64]) {
    let src = program(rel);
    let opt = compile_source(&src).unwrap_or_else(|e| panic!("optimized {rel}: {e:?}"));
    let raw = compile_source_unoptimized(&src).unwrap_or_else(|e| panic!("raw {rel}: {e:?}"));
    let a = evaluate(&opt, public, secret).unwrap_or_else(|e| panic!("run optimized {rel}: {e:?}"));
    let b = evaluate(&raw, public, secret).unwrap_or_else(|e| panic!("run raw {rel}: {e:?}"));
    assert_eq!(a, b, "the optimizer changed the output of {rel}");
}

#[test]
fn the_optimizer_preserves_behavior() {
    // Sequences with carried accumulators, the class the loop-rebinding bug lived in.
    same("lucas.zkl", &[], &[]);
    same("fib.zkl", &[], &[]);
    same("tribonacci.zkl", &[], &[]);
    same("factorial.zkl", &[], &[]);
    same("triangular.zkl", &[], &[]);
    same("sum_of_squares.zkl", &[], &[]);
    same("geometric.zkl", &[2], &[]);
    same("power.zkl", &[2], &[]);
    // Arithmetic and repeated subexpressions, the class CSE touches.
    same("cube.zkl", &[4], &[]);
    same("quartic.zkl", &[3], &[]);
    same("horner.zkl", &[2], &[]);
    same("norm_sq.zkl", &[2, 3, 4], &[]);
    // Arrays and constant tables.
    same("matmul2.zkl", &[], &[]);
    same("matvec.zkl", &[1, 1, 1], &[]);
    same("sumvec.zkl", &[1, 2, 3, 4, 5], &[]);
    same("dot.zkl", &[1, 2, 3, 4, 5, 6, 7, 8], &[]);
    same("schedule.zkl", &[7], &[]);
    // Functions and a full hash, exercising inlining and calls.
    same("mimc.zkl", &[123], &[]);
    same("merkle2.zkl", &[3, 5], &[]);
}
