/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The register file. Thirty-two lanes let a program hold more live values at once
//! than the earlier sixteen, which is what a width-12 permutation needs: its state
//! and the mixing matrix keep the old lanes live while the new ones are built. These
//! tests hold more than sixteen values live at the same time, which the old file
//! could not, and check the boundary is still a clean compile error rather than a
//! silent overrun.

use nonos_zkolang::{compile_source, prove_source_with_inputs, CompileError, REGS};
use std::fmt::Write as _;

// Build `input a0; ... input a{n-1}; output a0 + a1 + ... + a{n-1};`. Every input
// binds a register that stays live until the final sum, so the program holds `n`
// values live at once.
fn sum_of_inputs(n: usize) -> String {
    let mut src = String::new();
    for i in 0..n {
        write!(src, "input a{i}; ").unwrap();
    }
    src.push_str("output ");
    for i in 0..n {
        if i > 0 {
            src.push_str(" + ");
        }
        write!(src, "a{i}").unwrap();
    }
    src.push(';');
    src
}

#[test]
fn more_than_sixteen_live_values_now_fit() {
    // Twenty simultaneously live registers, which the sixteen-lane file rejected.
    let n = 20;
    let src = sum_of_inputs(n);
    let inputs: Vec<u64> = (0..n as u64).map(|i| i + 1).collect();
    let report = prove_source_with_inputs(&src, &inputs).expect("run");
    assert!(report.verified);
    assert_eq!(
        report.outputs,
        vec![(1..=n as u64).sum()],
        "1 + 2 + ... + 20"
    );
}

#[test]
fn the_file_fills_to_its_last_lane() {
    // Summing k inputs peaks at k + 1 live registers, the k inputs plus the one
    // accumulator, so a sum of `REGS - 1` inputs uses every lane at its peak.
    let n = REGS - 1;
    let src = sum_of_inputs(n);
    let inputs: Vec<u64> = (0..n as u64).map(|_| 1).collect();
    let report = prove_source_with_inputs(&src, &inputs).expect("run");
    assert!(report.verified);
    assert_eq!(
        report.outputs,
        vec![n as u64],
        "a one from each of the filled lanes"
    );
}

#[test]
fn overflowing_the_file_is_a_clean_error() {
    // One lane past the file is a typed compile error, never a silent overrun. A
    // sum of `REGS` inputs would need `REGS + 1` registers at its peak.
    let src = sum_of_inputs(REGS);
    assert!(matches!(
        compile_source(&src),
        Err(CompileError::TooManyRegisters)
    ));
}
