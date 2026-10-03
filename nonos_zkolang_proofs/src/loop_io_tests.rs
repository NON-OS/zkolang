/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Inputs, outputs and advice read inside loops. */

use nonos_zkolang::{compile_source, prove_source_with_inputs, CompileError};

#[test]
fn unrolling_past_the_machine_indices_is_an_error() {
    /*
     * The machine names inputs and outputs by sixteen-bit indices. A loop that unrolls past
     * them wrapped its counter onto indices already used, a panic in a debug build and a
     * program reading the wrong inputs in a release one.
     */
    let outputs = "for i in 0..40000 { output 1; }\nfor j in 0..30000 { output 2; }";
    assert_eq!(compile_source(outputs).err(), Some(CompileError::IoLimit));
    /* Each ordered comparison takes forty-nine advice bits from the same index space. */
    let advice = "input a;\ninput b;\nfor i in 0..1400 { let c = a < b; }\noutput a;";
    assert_eq!(compile_source(advice).err(), Some(CompileError::IoLimit));
}

#[test]
fn an_input_read_in_a_loop_reuses_its_register() {
    /*
     * Each iteration's `input x` shadows the last one. The shadowed register used to stay
     * held while the name was live, one register per iteration, so the thirty-third
     * iteration ran out of registers.
     */
    let src = "let acc = 0;\nfor i in 0..40 { input x; let acc = acc + x; }\noutput acc;";
    let inputs: Vec<u64> = (1..=40).collect();
    let report = prove_source_with_inputs(src, &inputs).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![820]);
    /* A secret the same way, and an input shadowing an array name reads the new input. */
    let secret = "for i in 0..40 { secret w; assert w - 7; }\noutput 1;";
    assert!(compile_source(secret).is_ok());
    let shadow = "let a = [1, 2];\ninput a;\noutput a;";
    let report = prove_source_with_inputs(shadow, &[9]).expect("run");
    assert_eq!(report.outputs, vec![9]);
}
