/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Register pressure that no order relieves: forty secret inputs, each used again after
 * their sum, need forty-one registers at once, and none can be read twice, so the program
 * is refused. As public inputs, which can be read again, the same program runs.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::codegen::CodegenError;
use nonos_zkolang::compiler::driver::{backend, BackendError};
use nonos_zkolang::compiler::ssa::{Builder, Inst, Ssa};

use crate::ssa_pressure_tests::outputs;

/** Forty inputs, public or secret, each multiplied by their sum and output. */
fn wide(public: bool) -> Ssa {
    let mut b = Builder::default();
    let xs: Vec<_> = (0..40).map(|i| b.emit(Inst::Input(i))).collect();
    let mut sum = b.konst(0);
    for &x in &xs {
        sum = b.add(sum, x);
    }
    for (i, &x) in xs.iter().enumerate() {
        let m = b.mul(x, sum);
        b.emit(Inst::Output(i as u16, m));
    }
    let (n_public, n_secret) = if public { (40, 0) } else { (0, 40) };
    Ssa {
        n_public,
        n_secret,
        n_outputs: 40,
        ..b.ssa
    }
}

#[test]
fn forty_secret_inputs_needed_at_once_are_refused() {
    let e = backend(&wide(false)).err();
    assert!(
        matches!(e, Some(BackendError::Codegen(CodegenError::Pressure(_)))),
        "{e:?}"
    );
    let public = wide(true);
    let c = backend(&public).expect("public inputs are read again");
    let inputs: Vec<Fp> = (1..=40).map(Fp::from_u64).collect();
    let (got, want) = outputs(&c, &public, &inputs);
    assert_eq!(got, want);
}
