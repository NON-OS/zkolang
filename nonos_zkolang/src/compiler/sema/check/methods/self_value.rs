/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `self` as a value, and a function of an `impl` block called as a method it is not. */

use alloc::format;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Ident};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `self`, the receiver of the method being checked. */
    pub(crate) fn self_value(&mut self, at: Span) -> TExpr {
        let Some(l) = self.lookup("self") else {
            let what = "`self` is a value only in a method, whose first parameter it names";
            let d = Diagnostic::error(Code::UNRESOLVED_NAME, what, at, "not in a method");
            self.sema.diags.push(d);
            return self.error(at);
        };
        self.read_local(l);
        let ty = self.locals.get(l.0 as usize).map_or(Types::ERROR, |x| x.ty);
        TExpr {
            kind: TExprKind::Local(l),
            ty,
            span: at,
        }
    }

    /** Report that `method` of `ty` takes no `self`, so is not called as a method. */
    pub(super) fn not_a_method(
        &mut self,
        ty: TyId,
        method: &Ident,
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        let (shown, name) = (self.show(ty), &method.name);
        let what = format!(
            "`{name}` is a function of `{shown}`, not a method; call it as `{shown}::{name}(..)`"
        );
        self.sema.diags.push(Diagnostic::error(
            Code::NO_FIELD,
            what,
            method.span,
            "not a method",
        ));
        args.iter().for_each(|a| {
            self.infer(a, None);
        });
        self.error(at)
    }

    /** Report generic arguments given to the method `method`, which takes none (E0701). */
    pub(super) fn no_generic_args(&mut self, method: &Ident) {
        let d = Diagnostic::error(
            Code::WRONG_GENERICS,
            format!("`{}` takes no generic arguments", method.name),
            method.span,
            "generic arguments given",
        );
        self.sema.diags.push(d);
    }
}
