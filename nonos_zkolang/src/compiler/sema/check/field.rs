/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Named fields (section 7.7), which no type this build checks has, and the report of a missing field. */

use alloc::boxed::Box;
use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Ident};
use crate::compiler::tir::{TExpr, TExprKind};

use super::structs::Key;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `a.name` on a struct with named fields. */
    pub(crate) fn named_field(&mut self, a: &'a Expr, name: &Ident, at: Span) -> TExpr {
        let a = self.infer(a, None);
        let Some((i, ty)) = self.field_of(a.ty, Key::Name(&name.name), at) else {
            return self.error(at);
        };
        TExpr {
            kind: TExprKind::TupleField(Box::new(a), i),
            ty,
            span: at,
        }
    }

    /** Report that type `t` has no field `name` (E0308). */
    pub(crate) fn no_field(&mut self, t: TyId, name: &str, at: Span) {
        let shown = self.show(t);
        let d = Diagnostic::error(
            Code::NO_FIELD,
            format!("`{shown}` has no field `{name}`"),
            at,
            "no such field",
        );
        self.sema.diags.push(d);
    }
}
