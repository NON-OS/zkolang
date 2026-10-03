/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Compile a checked program all the way to machine code and run it on the VM, for tests:
 * lower, back end, witness, VM. A run the program rejects is `None`.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::{backend, witness, Compiled};
use nonos_zkolang::compiler::lower::{lower_function, lower_program, LowerError};
use nonos_zkolang::compiler::tir::{FnId, TProgram};
use nonos_zkolang::Vm;

/** The compiled program of `p`'s `main`, panicking with the reason if it does not compile. */
pub(crate) fn compile(p: &TProgram) -> Compiled {
    try_compile(p).unwrap_or_else(|e| panic!("{e}"))
}

/** The compiled program of `p`'s `main`, or why it does not compile. */
pub(crate) fn try_compile(p: &TProgram) -> Result<Compiled, String> {
    let ssa = lower_program(p).map_err(|e| format!("lowering: {e:?}"))?;
    backend(&ssa).map_err(|e| format!("back end: {e:?}"))
}

/** The outputs of a run of `c` on `inputs`, with `n_public` public slots; `None` if rejected. */
pub(crate) fn run(c: &Compiled, inputs: &[Fp], n_public: usize) -> Option<Vec<Fp>> {
    let full = witness(c, inputs).ok()?;
    Vm::new()
        .run(&c.machine.ops, &full, n_public)
        .ok()
        .map(|t| t.public_outputs)
}

/** The field element standing for the integer `v`. */
pub(crate) fn fp(v: i128) -> Fp {
    let p = i128::from(nonos_stark::field::P);
    Fp::from_u64(v.rem_euclid(p) as u64)
}

/** The slots of the integer `v` of a type `bits` wide: one, or a 64-bit pattern's halves. */
pub(crate) fn slots(v: i128, bits: u32) -> Vec<Fp> {
    if bits <= 32 {
        return vec![fp(v)];
    }
    let pattern = v.rem_euclid(1i128 << 64);
    vec![fp(pattern & 0xFFFF_FFFF), fp(pattern >> 32)]
}

/**
 * The compiled program running the test `f` of `p`, or why it did not compile: whether the
 * reason is a form this build does not compile yet, and what it is.
 */
pub(crate) fn compile_test(p: &TProgram, f: FnId) -> Result<Compiled, (bool, String)> {
    let ssa = match lower_function(p, f) {
        Ok(ssa) => ssa,
        Err(LowerError::Unsupported(what, _)) => return Err((true, what.to_string())),
        Err(e) => return Err((false, format!("{e:?}"))),
    };
    backend(&ssa).map_err(|e| (false, format!("{e:?}")))
}
