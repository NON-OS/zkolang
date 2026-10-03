/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The prefix operators: `-` (section 7.1 and 7.2) and `!` (sections 7.4 and 7.5). */

use super::arith::int_neg;
use super::arith_field::field_neg;
use super::machine::Interp;
use super::{FailKind, Value};
use crate::compiler::sema::ty::TyKind;
use crate::compiler::syntax::ast::UnOp;
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::TExpr;

impl<'e> Interp<'e> {
    /** `op v`, `v` of the type of `operand`. */
    pub(super) fn unary(&self, op: UnOp, v: Value, operand: &TExpr) -> Result<Value, FailKind> {
        match (op, v, self.env.types().kind(operand.ty)) {
            (UnOp::Neg, Value::Field(x), _) => Ok(field_neg(x)),
            (UnOp::Neg, Value::Int(x), TyKind::Int(t)) => int_neg(x, *t),
            (UnOp::Not, Value::Bool(b), _) => Ok(Value::Bool(!b)),
            (UnOp::Not, Value::Int(x), TyKind::Int(t)) if t.signed() => Ok(Value::Int(!x)),
            (UnOp::Not, Value::Int(x), TyKind::Int(t)) => Ok(Value::Int(IntTy::max(*t) - x)),
            _ => Err(FailKind::Internal),
        }
    }
}
