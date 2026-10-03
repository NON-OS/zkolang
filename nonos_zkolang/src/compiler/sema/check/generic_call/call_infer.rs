/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A call of a generic function with constant parameters and no `::<..>` (section 10.5).
 * Each argument whose parameter's type names a constant parameter is checked first, on
 * its own, and its type gives the constants; the type parameters take variables. The
 * call is then checked as one for those arguments, the arguments checked first fitted to
 * the instance's parameter types.
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{FnId, TExpr};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `t(args)`, `t` named `name`, its generic arguments inferred. */
    pub(super) fn call_inferred(
        &mut self,
        t: FnId,
        name: &str,
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        match self.infer_generics(t, (Vec::new(), 0), args, at) {
            Some((generics, first)) => {
                self.call_template_with(t, (name, generics), args, first, at)
            }
            None => self.error(at),
        }
    }

    /** A variable for the type parameter `p` of the item `name`, at `at`. */
    pub(crate) fn fresh_param(&mut self, p: &str, name: &str, at: Span) -> GenArg {
        let what = format!("the type `{p}` of `{name}`");
        GenArg::Type(self.vars.fresh_general(&mut self.sema.types, at, what))
    }
}
