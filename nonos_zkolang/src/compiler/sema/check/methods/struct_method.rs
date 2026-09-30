/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A method call on a struct value: the method found through the struct's type. */

use super::super::cx::FnCx;
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, GenericArg, Ident, Param};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * `value.m(args)` on a struct: the method `m` of its `impl` blocks, generic arguments
     * refused, the receiver checked as `recv` from the diagnostics mark it carries.
     */
    pub(super) fn struct_method(
        &mut self,
        recv: (&'a Expr, TExpr, TyId, usize),
        (method, given): (&Ident, Option<&'a [GenericArg]>),
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        let Some((fid, impl_args)) = self.member_fn(recv.2, &method.name, method.span) else {
            args.iter().for_each(|a| {
                self.infer(a, None);
            });
            return self.error(at);
        };
        let is_method = self
            .sema
            .fns
            .get(fid.0 as usize)
            .and_then(|f| f.decl.params.first());
        if !matches!(is_method, Some(Param::SelfParam { .. })) {
            return self.not_a_method(recv.2, method, args, at);
        }
        let found = (fid, impl_args);
        let Some((g, first)) = self.member_call_generics(found, given, (args, 1), method.span)
        else {
            return self.error(at);
        };
        self.user_method((fid, g, first), recv, method, args, at)
    }
}
