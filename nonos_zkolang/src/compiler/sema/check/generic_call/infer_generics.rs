/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The generic arguments of a call written without `::<..>` (section 10.5), of a free
 * function or of an `impl` block's: each argument whose parameter's type names a constant
 * parameter is checked first, on its own, and its type gives the constants; the type
 * parameters take variables.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, GenericParam};
use crate::compiler::tir::{FnId, TArg};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The generic arguments of the template `t` called on `args`, which leave out its first
     * `skip` parameters: `before`, those of its `impl` block, then its own, inferred. Also
     * the arguments checked on the way; `None` once reported, every argument checked.
     */
    pub(crate) fn infer_generics(
        &mut self,
        t: FnId,
        (before, skip): (Vec<GenArg>, usize),
        args: &'a [Expr],
        at: Span,
    ) -> Option<(Vec<GenArg>, Vec<Option<TArg>>)> {
        let Some(decl) = self.sema.fns.get(t.0 as usize).map(|i| i.decl) else {
            self.check_args_then_error(args, at);
            return None;
        };
        let name = decl.name.name.as_str();
        let consts: Vec<&str> = decl
            .generics
            .iter()
            .filter_map(|g| match g {
                GenericParam::Const { name, .. } => Some(name.name.as_str()),
                GenericParam::Type(_) => None,
            })
            .collect();
        let mut values = vec![None; consts.len()];
        let first = self.args_first(t, (args, skip), (&consts, &mut values));
        let mut generics = before;
        let mut k = 0;
        for g in &decl.generics {
            match g {
                GenericParam::Type(p) => generics.push(self.fresh_param(&p.name, name, at)),
                GenericParam::Const { name: c, .. } => {
                    let Some(n) = values.get(k).copied().flatten() else {
                        self.not_inferred((&c.name, name), args, first, at);
                        return None;
                    };
                    generics.push(GenArg::Const(n));
                    k += 1;
                }
            }
        }
        Some((generics, first))
    }
}
