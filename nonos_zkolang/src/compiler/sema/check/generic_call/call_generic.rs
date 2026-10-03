/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Calls of a generic function (section 10.5): its generic arguments written with `::<..>`,
 * or else a variable for each type parameter, which the arguments and the type expected
 * of the call settle, and each constant parameter taken from the arguments' types. The
 * call names the template until the body settles; its rewrite then gives it the instance
 * for the arguments settled.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, GenericParam, Path};
use crate::compiler::tir::{FnId, TExpr};

impl<'s, 'a> FnCx<'s, 'a> {
    /** A call of the generic function `t`, which `p` names, on `args`. */
    pub(crate) fn call_generic(
        &mut self,
        t: FnId,
        p: &'a Path,
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        let given = p.segments.last().and_then(|s| s.generics.as_deref());
        let Some(info) = self.sema.fns.get(t.0 as usize) else {
            return self.check_args_then_error(args, at);
        };
        let def = info.def;
        let consts = info
            .decl
            .generics
            .iter()
            .any(|g| matches!(g, GenericParam::Const { .. }));
        if given.is_none() && consts {
            return self.call_inferred(t, p.last_name(), args, at);
        }
        let Some(generics) = self.generic_args(def, given, at) else {
            return self.check_args_then_error(args, at);
        };
        self.call_template(t, (p.last_name(), generics), args, at)
    }

    /** A call of the template `t`, named `name`, for `generics`, on `args`. */
    pub(crate) fn call_template(
        &mut self,
        t: FnId,
        named: (&str, Vec<GenArg>),
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        self.call_template_with(t, named, args, Vec::new(), at)
    }
}
