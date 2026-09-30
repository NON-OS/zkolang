/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A method call on a struct value: the method found through the struct's type. */

use super::super::cx::FnCx;
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Ident};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * `value.m(args)` on a struct: the method `m` of its `impl` blocks, generic arguments
     * refused, the receiver checked as `recv` from the diagnostics mark it carries.
     */
    pub(super) fn struct_method(
        &mut self,
        recv: (&'a Expr, TExpr, TyId, usize),
        (method, generic): (&Ident, bool),
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        if generic {
            self.no_generic_args(method);
        }
        let Some(fid) = self.member_fn(recv.2, &method.name, method.span) else {
            args.iter().for_each(|a| {
                self.infer(a, None);
            });
            return self.error(at);
        };
        self.user_method(fid, recv, method, args, at)
    }
}
