/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The language checking a run of itself. A zKolang circuit verifies a zKolang execution trace of
//! the register machine, using the same one-hot opcode gating the step AIR uses. The trace form
//! computes `(in0 + in1) * in2` by an add step then a multiply step; the register-machine form
//! reads operands from a register file and writes results back, proving the register dataflow. An
//! honest run proves; a forged result and a step that names two operations both have no proof. This
//! is zKolang attesting a zKolang execution, so it is the language running on itself.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_inputs};

fn stdlib_resolve(name: &str) -> Option<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    fs::read_to_string(base.join("stdlib").join(name)).ok()
}

fn load(rel: &str) -> String {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    let src = fs::read_to_string(base.join(rel)).expect("read");
    expand_includes(&src, &mut stdlib_resolve).expect("expand")
}

fn proves(src: &str, inputs: &[u64]) -> bool {
    prove_source_with_inputs(src, inputs)
        .map(|r| r.verified)
        .unwrap_or(false)
}

#[test]
fn a_run_verifies_and_binds_its_trace() {
    // in0 in1 in2 | s0_add s0_sub s0_mul t0 | s1_add s1_sub s1_mul t1
    // (3 + 5) * 2 = 16: step 0 is add, step 1 is multiply reading step 0's result.
    let src = load("examples/vm/verify_run.zkl");
    let honest = [3, 5, 2, 1, 0, 0, 8, 0, 0, 1, 16];
    let report = prove_source_with_inputs(&src, &honest).expect("run");
    assert!(report.verified, "an honest execution trace was rejected");
    assert_eq!(report.outputs, vec![16], "the run's output");

    // A wrong step result has no proof: the result binding fails.
    let mut wrong_result = honest;
    wrong_result[10] = 17;
    assert!(
        !proves(&src, &wrong_result),
        "a forged step result verified"
    );

    // A step naming two operations has no proof: the one-hot constraint fails.
    let mut two_ops = honest;
    two_ops[5] = 1;
    assert!(
        !proves(&src, &two_ops),
        "a step naming two operations verified"
    );
}

#[test]
fn a_register_machine_run_verifies() {
    // r0 r1 | s0: add sub mul, sa sb sd, res | s1: add sub mul, sa sb sd, res
    // r0 = r0 + r1 then r1 = r0 * r1 on (r0, r1) = (3, 5): the file ends (8, 40).
    let src = load("examples/vm/verify_registers.zkl");
    let honest = [3, 5, 1, 0, 0, 0, 1, 0, 8, 0, 0, 1, 0, 1, 1, 40];
    let report = prove_source_with_inputs(&src, &honest).expect("run");
    assert!(report.verified, "an honest register run was rejected");
    assert_eq!(report.outputs, vec![8, 40], "the final register file");

    // A wrong result in the second step has no proof.
    let mut wrong = honest;
    wrong[15] = 41;
    assert!(!proves(&src, &wrong), "a forged register result verified");

    // Reading the wrong source register breaks the operand binding.
    let mut wrong_src = honest;
    wrong_src[12] = 1;
    assert!(
        !proves(&src, &wrong_src),
        "a step reading the wrong register verified"
    );
}
