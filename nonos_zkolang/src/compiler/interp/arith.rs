/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Integer arithmetic (section 7.2): checked, so a result outside the type fails the run;
 * division truncates toward zero, and the remainder takes the sign of the dividend.
 */

use super::bits::shift;
use super::{FailKind, Value};
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::syntax::IntTy;

/** `a op b` for integers of type `t`; `b` is a `u32` for a shift. */
pub(super) fn int_op(op: BinOp, a: i128, b: i128, t: IntTy) -> Result<Value, FailKind> {
    let v = match op {
        BinOp::Add => a.checked_add(b),
        BinOp::Sub => a.checked_sub(b),
        BinOp::Mul => a.checked_mul(b),
        BinOp::Div | BinOp::Rem if b == 0 => return Err(FailKind::DivideByZero),
        BinOp::Rem if t.signed() && a == t.min() && b == -1 => return Err(FailKind::Overflow),
        BinOp::Div => a.checked_div(b),
        BinOp::Rem => a.checked_rem(b),
        BinOp::BitAnd => Some(a & b),
        BinOp::BitOr => Some(a | b),
        BinOp::BitXor => Some(a ^ b),
        BinOp::Shl | BinOp::Shr => return shift(a, b, op == BinOp::Shl, t).map(Value::Int),
        _ => return Ok(Value::Bool(compare(op, a.cmp(&b)))),
    };
    match v {
        Some(v) if t.contains(v) => Ok(Value::Int(v)),
        _ => Err(FailKind::Overflow),
    }
}

/** The result of the comparison `op` for operands that compare as `ord`. */
pub(super) fn compare(op: BinOp, ord: core::cmp::Ordering) -> bool {
    use core::cmp::Ordering::{Equal, Greater, Less};
    match op {
        BinOp::Eq => ord == Equal,
        BinOp::Ne => ord != Equal,
        BinOp::Lt => ord == Less,
        BinOp::Le => ord != Greater,
        BinOp::Gt => ord == Greater,
        BinOp::Ge => ord != Less,
        _ => false,
    }
}

/** `-a` for an integer of type `t`, which fails for the smallest signed value. */
pub(super) fn int_neg(a: i128, t: IntTy) -> Result<Value, FailKind> {
    let v = -a;
    if t.contains(v) {
        Ok(Value::Int(v))
    } else {
        Err(FailKind::Overflow)
    }
}
