/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Small hand-written SSA programs over two public inputs, compiled for gadget tests. */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::{backend, Compiled};
use nonos_zkolang::compiler::ssa::{Builder, Hint, Inst, Ssa, V};
use nonos_zkolang::Vm;

/** A program over two inputs outputting what `body` computes. */
pub(crate) fn compile(body: &dyn Fn(&mut Builder) -> V) -> Compiled {
    let mut b = Builder::default();
    let out = body(&mut b);
    b.emit(Inst::Output(0, out));
    let ssa = Ssa {
        n_public: 2,
        n_outputs: 1,
        ..b.ssa
    };
    backend(&ssa).expect("compiles")
}

pub(crate) fn accepts(c: &Compiled, full: &[Fp]) -> bool {
    Vm::new().run(&c.machine.ops, full, 2).is_ok()
}

/** The hint of advice slot `s` of `c`. */
pub(crate) fn hint_of(c: &Compiled, s: usize) -> Option<Hint> {
    c.machine.advice.get(s).copied()
}
