/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edges of proving edition 2026 programs: `MIN_ROWS` is the fewest rows the prover
 * hides a witness in, and a run takes and gives every leaf of tuples, arrays, negative
 * and 64-bit values.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::{prove, run, MIN_ROWS};
use nonos_zkolang::{prove_program_hidden, Op, RunError};

use crate::prove_2026_tests::{built, SEED};

#[test]
fn min_rows_is_the_fewest_the_prover_hides_a_witness_in() {
    let program = |rows: usize| {
        let mut ops = vec![Op::Imm { d: 0, v: Fp::ZERO }; rows - 1];
        ops.push(Op::Halt);
        ops
    };
    let short = prove_program_hidden(&program(MIN_ROWS - 1), &[], 0, &SEED);
    assert!(matches!(short, Err(RunError::TraceTooSmallToHide { .. })));
    let r = prove_program_hidden(&program(MIN_ROWS), &[], 0, &SEED).expect("proves");
    assert!(r.verified);
}

#[test]
fn a_run_takes_and_gives_every_leaf() {
    let src = "fn main(a: public i64, b: secret [u8; 2], c: secret i8) -> public (i64, bool, u64, i8) {\n    declassify((a - b[0] as i64, b[1] > 3, (b[0] as u64) * 4294967296, c - 1))\n}\n";
    let b = built(src);
    let want = vec![-12, 1, 7 * 4294967296, -4];
    assert_eq!(run(&b, &[-5], &[7, 9, -3]), Ok(want.clone()));
    let p = prove(&b, &[-5], &[7, 9, -3], &SEED).expect("proves");
    assert_eq!(p.outputs, want);
    assert_eq!(run(&b, &[i128::from(i64::MIN)], &[1, 0, 0]).ok(), None);
}

#[test]
fn forty_secret_values_needed_at_once_are_refused_with_e0801() {
    let src = |label: &str| {
        format!("fn main(xs: {label} [field; 40]) -> public field {{\n    let mut s = 0;\n    for x in xs {{\n        s = s + x;\n    }}\n    let mut t = 0;\n    for x in xs {{\n        t = t + x * s;\n    }}\n    declassify(t)\n}}\n")
    };
    let secret = src("secret");
    let mut map = nonos_zkolang::compiler::source::SourceMap::new();
    let files = nonos_zkolang::compiler::syntax::load::NoFiles;
    let codes = match nonos_zkolang::compiler::driver::build(&mut map, &files, ("p.zkl", secret)) {
        Err(d) => d.items().iter().map(|d| d.code.0).collect::<Vec<_>>(),
        Ok(_) => Vec::new(),
    };
    assert_eq!(codes, vec!["E0801"]);
    let b = built(&src("public"));
    let xs: Vec<i128> = (1..=40).collect();
    assert_eq!(run(&b, &xs, &[]), Ok(vec![820 * 820]));
}
