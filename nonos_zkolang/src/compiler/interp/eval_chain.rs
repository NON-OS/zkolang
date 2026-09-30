/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Operators: a chain of one precedence, left to right, with `&&` and `||` evaluating an
 * operand only when the value so far does not decide the result (section 7.4); and the
 * prefix operators.
 */

use super::arith::{compare, int_op};
use super::arith_field::field_op;
use super::machine::{fail, Eval, Interp};
use super::{FailKind, Value};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::tir::TExpr;

impl<'e> Interp<'e> {
    /** `first op1 e1 op2 e2 ...`, left to right. */
    pub(super) fn chain(&mut self, first: &TExpr, links: &[(BinOp, TExpr)]) -> Eval {
        let mut acc = self.eval(first)?;
        for (op, e) in links {
            let decided = match op {
                BinOp::And => !acc.bool(),
                BinOp::Or => acc.bool(),
                _ => false,
            };
            if decided {
                continue;
            }
            let rhs = self.eval(e)?;
            acc = self
                .binop(*op, &acc, &rhs, first.ty)
                .map_err(|k| fail(k, e.span))?;
        }
        Ok(acc)
    }

    /** `a op b`, `a` of type `lhs`. */
    pub(super) fn binop(
        &self,
        op: BinOp,
        a: &Value,
        b: &Value,
        lhs: TyId,
    ) -> Result<Value, FailKind> {
        match (op, a, b) {
            (BinOp::And | BinOp::Or, _, _) => Ok(b.clone()),
            (BinOp::Eq, _, _) => Ok(Value::Bool(a == b)),
            (BinOp::Ne, _, _) => Ok(Value::Bool(a != b)),
            (_, Value::Field(x), Value::Field(y)) => field_op(op, *x, *y),
            (_, Value::Bool(x), Value::Bool(y)) => Ok(Value::Bool(match op {
                BinOp::BitAnd => x & y,
                BinOp::BitOr => x | y,
                BinOp::BitXor => x ^ y,
                _ => compare(op, x.cmp(y)),
            })),
            (_, Value::Int(x), _) => match self.env.types().kind(lhs) {
                TyKind::Int(t) => int_op(op, *x, b.int(), *t),
                _ => Err(FailKind::Internal),
            },
            _ => Err(FailKind::Internal),
        }
    }
}
