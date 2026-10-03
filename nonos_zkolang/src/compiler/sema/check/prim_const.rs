/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The constants `MIN`, `MAX` and `BITS` of an integer type (section 18.3). An associated
 * function named without a call is reported as not a value.
 */

use alloc::format;

use super::cx::FnCx;
use super::prim::prim_path;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyKind, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Path;
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::{TExpr, TExprKind, TLit};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The constant a path over a primitive type names, if the path is over one. */
    pub(crate) fn prim_const(&mut self, p: &Path, at: Span) -> Option<TExpr> {
        let (ty, item) = prim_path(p)?;
        let int = match self.sema.types.kind(ty) {
            TyKind::Int(i) => Some(*i),
            _ => None,
        };
        let (value, ty) = match (item, int) {
            ("MIN", Some(i)) => (i.min(), ty),
            ("MAX", Some(i)) => (i.max(), ty),
            ("BITS", Some(i)) => (i128::from(i.bits()), Types::int(IntTy::U32)),
            (name, _) => {
                let shown = self.sema.types.display(ty);
                let d = if matches!(name, "checked_from" | "wrapping_from" | "from_le_bits") {
                    Diagnostic::error(
                        Code::WRONG_KIND,
                        format!("`{shown}::{name}` is a function, not a value"),
                        at,
                        "not a value",
                    )
                    .with_help(format!("call it: `{shown}::{name}(..)`"))
                } else {
                    Diagnostic::error(
                        Code::NO_FIELD,
                        format!("`{shown}` has no associated item `{name}`"),
                        at,
                        "no such item",
                    )
                };
                self.sema.diags.push(d);
                return Some(self.error(at));
            }
        };
        Some(TExpr {
            kind: TExprKind::Lit(TLit::Int(value)),
            ty,
            span: at,
        })
    }
}
