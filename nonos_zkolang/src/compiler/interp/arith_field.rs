/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `field` arithmetic (section 7.1): modular, with division by the inverse. */

use nonos_stark::field::Fp;

use super::arith::compare;
use super::{FailKind, Value};
use crate::compiler::syntax::ast::BinOp;

/** `a op b` for `field` elements. */
pub(super) fn field_op(op: BinOp, a: u64, b: u64) -> Result<Value, FailKind> {
    let (x, y) = (Fp::from_u64(a), Fp::from_u64(b));
    let v = match op {
        BinOp::Add => x + y,
        BinOp::Sub => x - y,
        BinOp::Mul => x * y,
        BinOp::Div if b == 0 => return Err(FailKind::DivideByZero),
        BinOp::Div => x * y.inv(),
        _ => return Ok(Value::Bool(compare(op, a.cmp(&b)))),
    };
    Ok(Value::Field(v.value()))
}

/** `-a` for a `field` element. */
pub(super) fn field_neg(a: u64) -> Value {
    Value::Field((-Fp::from_u64(a)).value())
}

/** `a.pow(k)` for a `field` element. */
pub(super) fn field_pow(a: u64, k: u64) -> Value {
    Value::Field(Fp::from_u64(a).pow(k).value())
}

/** `a.inv()`, which fails for zero. */
pub(super) fn field_inv(a: u64) -> Result<Value, FailKind> {
    if a == 0 {
        return Err(FailKind::InverseOfZero);
    }
    Ok(Value::Field(Fp::from_u64(a).inv().value()))
}

/** The `field` element an integer stands for: `x mod p`. */
pub(super) fn to_field(x: i128) -> u64 {
    let p = i128::from(nonos_stark::field::P);
    x.rem_euclid(p) as u64
}
