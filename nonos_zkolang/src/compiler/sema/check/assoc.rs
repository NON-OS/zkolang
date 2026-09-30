/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Associated functions of the primitive types (section 18.3): `T::checked_from(e)`,
 * `T::wrapping_from(e)` and `T::from_le_bits(bits)`.
 */

use alloc::format;
use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{Builtin, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `ty::name(args)`. */
    pub(crate) fn assoc_call(&mut self, ty: TyId, name: &str, args: &'a [Expr], at: Span) -> TExpr {
        let (b, bits) = match (name, self.kind(ty)) {
            ("checked_from", TyKind::Int(_) | TyKind::Bool) => (Builtin::CheckedFrom, None),
            ("wrapping_from", TyKind::Int(_)) => (Builtin::WrappingFrom, None),
            ("from_le_bits", TyKind::Int(i)) => (Builtin::FromLeBits, Some(i.bits())),
            ("from_le_bits", TyKind::Field) => (Builtin::FromLeBits, Some(64)),
            _ => {
                let shown = self.show(ty);
                let d = Diagnostic::error(
                    Code::NO_FIELD,
                    format!("`{shown}` has no associated function `{name}`"),
                    at,
                    "no such function",
                );
                self.sema.diags.push(d);
                return self.check_args_then_error(args, at);
            }
        };
        if args.len() != 1 {
            let d = Diagnostic::error(
                Code::WRONG_ARITY,
                format!("`{name}` takes 1 argument, not {}", args.len()),
                at,
                "wrong number of arguments",
            );
            self.sema.diags.push(d);
            return self.check_args_then_error(args, at);
        }
        let operands: Vec<TExpr> = args
            .iter()
            .map(|a| match bits {
                Some(n) => {
                    let want = self.bits_ty(n);
                    self.expr(a, Some(want))
                }
                None => self.int_or_field(a),
            })
            .collect();
        TExpr {
            kind: TExprKind::Builtin(b, operands),
            ty,
            span: at,
        }
    }
}
