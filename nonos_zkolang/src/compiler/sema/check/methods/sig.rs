/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The built-in methods of each type: the operation, its parameters and its result. An
 * array's `len` is its type's length, a constant, and the receiver is not evaluated.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Ident};
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::{Builtin, TExpr, TExprKind, TLit};

/** A method's operation, parameter types and result type. */
type Sig = (Builtin, Vec<TyId>, TyId);

impl<'s, 'a> FnCx<'s, 'a> {
    /** The method `method` of a receiver of type `rt`, or the expression the call is. */
    pub(super) fn method_sig(
        &mut self,
        receiver: &'a Expr,
        method: &Ident,
        rt: TyId,
        args: &'a [Expr],
        at: Span,
    ) -> Result<Sig, TExpr> {
        let (u32t, u64t) = (Types::int(IntTy::U32), Types::int(IntTy::U64));
        Ok(match (method.name.as_str(), self.kind(rt)) {
            (_, TyKind::Error) => return Err(self.check_args_then_error(args, at)),
            ("wrapping_add", TyKind::Int(_)) => (Builtin::WrappingAdd, vec![rt], rt),
            ("wrapping_sub", TyKind::Int(_)) => (Builtin::WrappingSub, vec![rt], rt),
            ("wrapping_mul", TyKind::Int(_)) => (Builtin::WrappingMul, vec![rt], rt),
            ("wrapping_neg", TyKind::Int(_)) => (Builtin::WrappingNeg, Vec::new(), rt),
            ("min", TyKind::Int(_)) => (Builtin::Min, vec![rt], rt),
            ("max", TyKind::Int(_)) => (Builtin::Max, vec![rt], rt),
            ("pow", TyKind::Int(_)) => (Builtin::Pow, vec![u32t], rt),
            ("pow", TyKind::Field) => (Builtin::Pow, vec![u64t], rt),
            ("inv", TyKind::Field) => (Builtin::Inv, Vec::new(), rt),
            ("to_le_bits", TyKind::Int(i)) => {
                (Builtin::ToLeBits, Vec::new(), self.bits_ty(i.bits()))
            }
            ("to_le_bits", TyKind::Field) => (Builtin::ToLeBits, Vec::new(), self.bits_ty(64)),
            ("len", TyKind::Array(_, n)) => {
                self.arity(method, 0, args, at);
                let kind = TExprKind::Lit(TLit::Int(i128::from(n)));
                let ty = Types::int(IntTy::Usize);
                return Err(TExpr { kind, ty, span: at });
            }
            _ => return Err(self.no_method(receiver, method, rt, args, at)),
        })
    }
}
