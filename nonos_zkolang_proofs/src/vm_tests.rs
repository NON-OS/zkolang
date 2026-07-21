/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The language checking a run of itself. A zKolang circuit verifies a two-step execution
//! trace of the register machine, computing `(in0 + in1) * in2` by an add step then a multiply
//! step, using the same one-hot opcode gating the step AIR uses. An honest trace proves; a
//! forged step result and a step that names two operations both have no proof. This is zKolang
//! attesting a zKolang execution, so it is the language running on itself.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_inputs};

fn stdlib_resolve(name: &str) -> Option<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    fs::read_to_string(base.join("stdlib").join(name)).ok()
}

fn circuit() -> String {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    let src = fs::read_to_string(base.join("examples/vm/verify_run.zkl")).expect("read");
    expand_includes(&src, &mut stdlib_resolve).expect("expand")
}

fn proves(inputs: &[u64]) -> bool {
    prove_source_with_inputs(&circuit(), inputs)
        .map(|r| r.verified)
        .unwrap_or(false)
}

#[test]
fn a_run_verifies_and_binds_its_trace() {
    // in0 in1 in2 | s0_add s0_sub s0_mul t0 | s1_add s1_sub s1_mul t1
    // (3 + 5) * 2 = 16: step 0 is add, step 1 is multiply reading step 0's result.
    let honest = [3, 5, 2, 1, 0, 0, 8, 0, 0, 1, 16];
    let report = prove_source_with_inputs(&circuit(), &honest).expect("run");
    assert!(report.verified, "an honest execution trace was rejected");
    assert_eq!(report.outputs, vec![16], "the run's output");

    // A wrong step result has no proof: the result binding fails.
    let mut wrong_result = honest;
    wrong_result[10] = 17;
    assert!(!proves(&wrong_result), "a forged step result verified");

    // A step naming two operations has no proof: the one-hot constraint fails.
    let mut two_ops = honest;
    two_ops[5] = 1;
    assert!(!proves(&two_ops), "a step naming two operations verified");
}
