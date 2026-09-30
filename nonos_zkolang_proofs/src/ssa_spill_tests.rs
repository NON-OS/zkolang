/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Register pressure: seventy values, each computed from the one before, all live until
 * the end, so most must be copied into the advice and read back. The machine program must
 * still compute every one, and every copy must be pinned to its value.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::{backend, witness};
use nonos_zkolang::compiler::ssa::eval::eval;
use nonos_zkolang::compiler::ssa::{Builder, Inst, Ssa};
use nonos_zkolang::Vm;

#[test]
fn seventy_live_values_are_spilled_and_read_back() {
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
    let ssa = Ssa {
        n_public: 2,
        n_outputs: vals.len() as u16,
        ..b.ssa
    };
    let c = backend(&ssa).expect("compiles");
    assert!(
        c.machine.advice.len() > 30,
        "only {} copies",
        c.machine.advice.len()
    );
    let inputs = [Fp::from_u64(3), Fp::from_u64(7)];
    let want = eval(&ssa, &inputs).expect("runs").outputs;
    let full = witness(&c, &inputs).expect("witness");
    let got = Vm::new()
        .run(&c.machine.ops, &full, 2)
        .map(|t| t.public_outputs);
    assert_eq!(got.ok(), Some(want));
    assert!(crate::ssa_mutate::unpinned(&c.machine.ops, &full, 2, 0).is_empty());
}
