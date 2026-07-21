/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The example corpus, proven. Every program under examples/ is a zKolang source
//! file, and each one here is compiled from disk, run, and its trace proven. The
//! ones with a closed-form answer check their public output; the rest check the
//! proof verifies and the program is a real, provable statement.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source, prove_source_with_inputs};

// Read an example program by name and resolve its includes from stdlib and examples, the
// way the command-line tool does, so an example may draw on the standard library.
fn program(name: &str) -> String {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.pop();
    let path = root.join("examples").join(name);
    let src = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let mut resolve = |inc: &str| {
        for dir in ["stdlib", "examples"] {
            if let Ok(s) = fs::read_to_string(root.join(dir).join(inc)) {
                return Some(s);
            }
        }
        None
    };
    expand_includes(&src, &mut resolve).unwrap_or_else(|e| panic!("expand {name}: {e:?}"))
}

#[test]
fn cube_of_a_public_input() {
    let report = prove_source_with_inputs(&program("cube.zkl"), &[4]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![64], "4^3");
}

#[test]
fn horner_evaluates_the_polynomial() {
    // 1 + 2x + 3x^2 at x = 2 is 1 + 4 + 12 = 17.
    let report = prove_source_with_inputs(&program("horner.zkl"), &[2]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![17]);
}

#[test]
fn fibonacci_reaches_the_tenth_number() {
    let report = prove_source(&program("fib.zkl")).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![55], "fib(10)");
}

#[test]
fn the_schedule_program_runs() {
    let report = prove_source_with_inputs(&program("schedule.zkl"), &[7]).expect("run");
    assert!(report.verified);
}

#[test]
fn the_merkle_node_is_a_provable_statement() {
    // A correct internal node over two children. Distinct children give a distinct
    // node, so the compression is not collapsing its inputs.
    let a = prove_source_with_inputs(&program("merkle2.zkl"), &[3, 5]).expect("run");
    let b = prove_source_with_inputs(&program("merkle2.zkl"), &[5, 3]).expect("run");
    assert!(a.verified && b.verified);
    assert_ne!(
        a.outputs, b.outputs,
        "left and right were not distinguished"
    );
}

#[test]
fn the_range_program_compiles() {
    // The range proof takes a private witness, so it is exercised in witness_tests;
    // here we only confirm the committed file is a well-formed program.
    assert!(nonos_zkolang::compile_source(&program("range.zkl")).is_ok());
}

#[test]
fn dot_product_over_arrays() {
    let report =
        prove_source_with_inputs(&program("dot.zkl"), &[1, 2, 3, 4, 5, 6, 7, 8]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![70], "1*5 + 2*6 + 3*7 + 4*8");
}

#[test]
fn matrix_times_vector() {
    let report = prove_source_with_inputs(&program("matvec.zkl"), &[1, 1, 1]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![6, 15, 24], "row sums of the matrix");
}

#[test]
fn sum_of_a_vector() {
    let report = prove_source_with_inputs(&program("sumvec.zkl"), &[1, 2, 3, 4, 5]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![15]);
}

#[test]
fn a_power_by_repeated_multiplication() {
    let report = prove_source_with_inputs(&program("power.zkl"), &[2]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![1024], "2^10");
}

#[test]
fn factorial_over_a_range() {
    let report = prove_source(&program("factorial.zkl")).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![720], "6!");
}

#[test]
fn a_geometric_series() {
    let report = prove_source_with_inputs(&program("geometric.zkl"), &[2]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![255], "1 + 2 + 4 + ... + 128");
}
