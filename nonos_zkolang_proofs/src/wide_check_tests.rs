/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A range check of 64 bits or more is outside its gadget's domain and fails in the SSA
 * semantics; the compiled program fails too, whether or not the 64 field bits of the value
 * were read just before it, and however large the value's known bound.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::{backend, witness};
use nonos_zkolang::compiler::ssa::eval::eval;
use nonos_zkolang::compiler::ssa::{Builder, Inst, Ssa};
use nonos_zkolang::Vm;

/** `x` checked below `2^n`, after reading its field bit 3 if `field_bits`. */
fn program(n: u8, field_bits: bool) -> Ssa {
    let mut b = Builder::default();
    let x = b.emit(Inst::Input(0));
    let mut outputs = 0;
    if field_bits {
        let f = b.emit(Inst::FieldBit(x, 3));
        b.emit(Inst::Output(0, f));
        outputs += 1;
    }
    b.emit(Inst::RangeCheck(x, n));
    b.emit(Inst::Output(outputs, x));
    let mut ssa = b.ssa;
    ssa.n_public = 1;
    ssa.n_outputs = outputs + 1;
    ssa
}

#[test]
fn a_range_check_of_64_bits_fails_compiled_as_in_the_semantics() {
    let inputs = [Fp::from_u64(5)];
    for (n, field_bits) in [(64, false), (64, true), (70, false), (32, true)] {
        let ssa = program(n, field_bits);
        let holds = eval(&ssa, &inputs).is_ok();
        assert_eq!(holds, n < 64, "the semantics, {n} bits");
        let compiled = backend(&ssa).unwrap_or_else(|e| panic!("{n} bits: {e:?}"));
        let run = witness(&compiled, &inputs).ok();
        let accepted =
            run.is_some_and(|full| Vm::new().run(&compiled.machine.ops, &full, 1).is_ok());
        assert_eq!(accepted, holds, "{n} bits, field bits read: {field_bits}");
    }
}
