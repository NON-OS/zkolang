/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A binary operator on two values of the type of `lhs`: equality on any type, and the
 * arithmetic, bitwise and ordering operators on `bool`, `field` and the integers.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::ast::BinOp;

impl<'p> Lower<'p> {
    /** `a op b`, both of type `ty`, written at `at`. */
    pub(super) fn binop(&mut self, op: BinOp, a: &[V], b: &[V], ty: TyId, at: Span) -> L<Vec<V>> {
        if matches!(op, BinOp::Eq | BinOp::Ne) {
            let mut all = self.b.konst(1);
            for (&x, &y) in a.iter().zip(b) {
                let e = self.b.emit(Inst::Eq(x, y));
                all = self.and(all, e);
            }
            return Ok(alloc::vec![if op == BinOp::Eq {
                all
            } else {
                self.b.not(all)
            }]);
        }
        let (Some(&x), Some(&y)) = (a.first(), b.first()) else {
            return Err(LowerError::Unsupported("an operator on an empty value", at));
        };
        let v = match self.p.types.kind(ty).clone() {
            TyKind::Bool => self.bool_op(op, x, y),
            TyKind::Field => self.field_op(op, x, y),
            TyKind::Int(t) if t.bits() <= 32 => self.int_op(op, x, y, t)?,
            TyKind::Int(t) => return self.int64_op(op, a, b, t, at),
            _ => return Err(LowerError::Unsupported("an operator on this type", at)),
        };
        Ok(alloc::vec![v])
    }
}
