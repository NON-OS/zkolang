/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Calls of a generic function (section 10.5): its generic arguments written with `::<..>`,
 * or else a variable for each type parameter, which the arguments and the type expected
 * of the call settle. The call names the template until the body settles; its rewrite
 * then gives it the instance for the arguments settled.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Path};
use crate::compiler::tir::{FnId, TArg, TExpr, TExprKind};

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
        let Some(def) = self.sema.fns.get(t.0 as usize).map(|i| i.def) else {
            return self.check_args_then_error(args, at);
        };
        let Some(generics) = self.generic_args(def, given, at) else {
            return self.check_args_then_error(args, at);
        };
        self.call_template(t, (p.last_name(), generics), args, at)
    }

    /** A call of the template `t`, named `name`, for `generics`, on `args`. */
    pub(crate) fn call_template(
        &mut self,
        t: FnId,
        (name, generics): (&str, Vec<GenArg>),
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        let sig = self.sema.sig_with(t, &generics);
        self.arity_of(name, sig.params.len(), args.len(), at);
        let targs: Vec<TArg> = args
            .iter()
            .enumerate()
            .map(|(i, a)| self.arg(a, sig.params.get(i)))
            .collect();
        self.check_disjoint(&targs);
        self.pending.push((at, generics));
        TExpr {
            kind: TExprKind::Call(t, targs),
            ty: sig.ret,
            span: at,
        }
    }
}
