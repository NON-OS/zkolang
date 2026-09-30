/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The generic arguments of a call of an `impl` block's function (section 10.5): one with
 * constant parameters of its own, called without `::<..>`, has them taken from the
 * arguments' types as a free function does; any other as `member_generics` gives them.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, GenericArg, GenericParam};
use crate::compiler::tir::{FnId, TArg};

/** A template's arguments, `None` for a function that is none, and the arguments checked. */
type Settled = (Option<Vec<GenArg>>, Vec<Option<TArg>>);

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The generic arguments of `fid`, given `impl_args` by its type and `given` written, for
     * a call on `args` that leave out its first `skip` parameters; `None` once reported,
     * every argument checked.
     */
    pub(crate) fn member_call_generics(
        &mut self,
        (fid, impl_args): (FnId, Vec<GenArg>),
        given: Option<&'a [GenericArg]>,
        (args, skip): (&'a [Expr], usize),
        at: Span,
    ) -> Option<Settled> {
        let own_const = self.sema.fns.get(fid.0 as usize).is_some_and(|i| {
            let consts = i.decl.generics.iter();
            i.template
                && consts
                    .clone()
                    .any(|g| matches!(g, GenericParam::Const { .. }))
        });
        if given.is_none() && own_const {
            let (g, first) = self.infer_generics(fid, (impl_args, skip), args, at)?;
            return Some((Some(g), first));
        }
        let Some(g) = self.member_generics((fid, impl_args), given, at) else {
            self.check_args_then_error(args, at);
            return None;
        };
        Some((g, Vec::new()))
    }
}
