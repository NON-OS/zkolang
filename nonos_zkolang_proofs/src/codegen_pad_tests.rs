/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Padding: every compiled program has at least `MIN_ROWS` rows, its padding passes the
 * verifier, and padding that is not a constant, or that overwrites a register a later
 * instruction reads, is rejected.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::codegen::{verify, Origin};
use nonos_zkolang::compiler::driver::{backend, Compiled, MIN_ROWS};
use nonos_zkolang::compiler::ssa::{Builder, Inst, Ssa};
use nonos_zkolang::Op;

/** `x * x` of one public input, compiled. */
fn square() -> Compiled {
    let mut b = Builder::default();
    let x = b.emit(Inst::Input(0));
    let y = b.mul(x, x);
    b.emit(Inst::Output(0, y));
    backend(&Ssa {
        n_public: 1,
        n_outputs: 1,
        ..b.ssa
    })
    .expect("compiles")
}

#[test]
fn a_short_program_is_padded_and_still_verifies() {
    let c = square();
    assert_eq!(c.machine.ops.len(), MIN_ROWS);
    assert_eq!(c.machine.origins.last(), Some(&Origin::Halt));
    assert!(c.machine.origins.contains(&Origin::Pad));
    assert_eq!(verify(&c.ssa, &c.machine), Ok(()));
}

#[test]
fn padding_that_computes_is_rejected() {
    let c = square();
    let at = c.machine.origins.iter().position(|o| *o == Origin::Pad);
    let mut m = c.machine.clone();
    let add = Op::Add { d: 1, a: 1, b: 1 };
    m.ops[at.expect("padded")] = add;
    let e = verify(&c.ssa, &m).expect_err("rejected");
    assert_eq!(e.why, "padding that is not a constant");
}

#[test]
fn padding_that_clobbers_a_register_read_later_is_rejected() {
    let c = square();
    let mut ops = c.machine.ops.iter().enumerate();
    let out = ops.find_map(|(k, op)| match op {
        Op::Out { a, .. } => Some((k, *a)),
        _ => None,
    });
    let (at, d) = out.expect("an output");
    let mut m = c.machine.clone();
    m.ops.insert(at, Op::Imm { d, v: Fp::ZERO });
    m.origins.insert(at, Origin::Pad);
    assert!(verify(&c.ssa, &m).is_err());
}
