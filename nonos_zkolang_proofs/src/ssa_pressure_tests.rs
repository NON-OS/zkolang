/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Register pressure: seventy values, each computed from the one before and each output,
 * need few registers once each output follows its value.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::{backend, witness, Compiled};
use nonos_zkolang::compiler::ssa::eval::eval;
use nonos_zkolang::compiler::ssa::{Builder, Inst, Ssa};
use nonos_zkolang::Vm;

/** The outputs of `c` on `inputs`, and of the SSA program it was compiled from. */
pub(crate) fn outputs(c: &Compiled, ssa: &Ssa, inputs: &[Fp]) -> (Vec<Fp>, Vec<Fp>) {
    let want = eval(ssa, inputs).expect("runs").outputs;
    let full = witness(c, inputs).expect("witness");
    let n = usize::from(ssa.n_public);
    let got = Vm::new().run(&c.machine.ops, &full, n).expect("accepts");
    (got.public_outputs, want)
}

#[test]
fn seventy_chained_values_need_few_registers() {
    let mut b = Builder::default();
    let (x, y) = (b.emit(Inst::Input(0)), b.emit(Inst::Input(1)));
    let mut vals = vec![x];
    for _ in 0..70 {
        let last = *vals.last().expect("a value");
        let m = b.mul(last, x);
        vals.push(b.add(m, y));
    }
    for (i, v) in vals.iter().enumerate() {
        b.emit(Inst::Output(i as u16, *v));
    }
    let n_outputs = vals.len() as u16;
    let ssa = Ssa {
        n_public: 0,
        n_secret: 2,
        n_outputs,
        ..b.ssa
    };
    let c = backend(&ssa).expect("compiles");
    let (got, want) = outputs(&c, &ssa, &[Fp::from_u64(3), Fp::from_u64(7)]);
    assert_eq!(got, want);
    assert!(crate::codegen_reads_tests::rereads(&c.machine.ops, 0).is_empty());
}
