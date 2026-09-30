/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Literals. An integer literal with a suffix has that type; one without takes the type
 * expected where it stands, or else a variable that the rest of the body settles
 * (section 5.6). Whether its value fits is checked once its type is known.
 */

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Lit;
use crate::compiler::tir::{TExpr, TExprKind, TLit};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The literal `l`, where `want` is expected. */
    pub(crate) fn lit(&mut self, l: &Lit, want: Option<TyId>, span: Span) -> TExpr {
        let (lit, ty) = match l {
            Lit::Bool { value, .. } => (TLit::Bool(*value), Types::BOOL),
            Lit::Int { value, suffix, .. } => (
                TLit::Int(i128::from(*value)),
                self.int_lit_ty(*suffix, want),
            ),
            Lit::Str { .. } => {
                let d = Diagnostic::error(
                    Code::MISMATCHED_TYPES,
                    "a string is not a value",
                    span,
                    "a string literal",
                )
                .with_help("strings stand only in `assert` messages and attributes");
                self.sema.diags.push(d);
                return self.error(span);
            }
        };
        TExpr {
            kind: TExprKind::Lit(lit),
            ty,
            span,
        }
    }

    /** The type of an integer literal with `suffix`, where `want` is expected. */
    pub(super) fn int_lit_ty(
        &mut self,
        suffix: Option<crate::compiler::syntax::IntTy>,
        want: Option<TyId>,
    ) -> TyId {
        if let Some(s) = suffix {
            return Types::int(s);
        }
        match want.map(|w| (w, self.kind(w))) {
            Some((w, TyKind::Int(_) | TyKind::Field | TyKind::Var(_))) => self.resolve(w),
            _ => self.vars.fresh(&mut self.sema.types, false),
        }
    }
}
