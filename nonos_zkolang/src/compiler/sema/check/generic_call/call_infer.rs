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
use alloc::vec;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, GenericParam};
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
        let Some(decl) = self.sema.fns.get(t.0 as usize).map(|i| i.decl) else {
            return self.check_args_then_error(args, at);
        };
        let consts: Vec<&str> = decl
            .generics
            .iter()
            .filter_map(|g| match g {
                GenericParam::Const { name, .. } => Some(name.name.as_str()),
                GenericParam::Type(_) => None,
            })
            .collect();
        let mut values = vec![None; consts.len()];
        let first = self.args_first(t, args, (&consts, &mut values));
        let mut generics = Vec::with_capacity(decl.generics.len());
        let mut k = 0;
        for g in &decl.generics {
            match g {
                GenericParam::Type(p) => generics.push(self.fresh_param(&p.name, name, at)),
                GenericParam::Const { name: c, .. } => {
                    let Some(n) = values.get(k).copied().flatten() else {
                        return self.not_inferred((&c.name, name), args, first, at);
                    };
                    generics.push(GenArg::Const(n));
                    k += 1;
                }
            }
        }
        self.call_template_with(t, (name, generics), args, first, at)
    }

    /** A variable for the type parameter `p` of the item `name`, at `at`. */
    pub(crate) fn fresh_param(&mut self, p: &str, name: &str, at: Span) -> GenArg {
        let what = format!("the type `{p}` of `{name}`");
        GenArg::Type(self.vars.fresh_general(&mut self.sema.types, at, what))
    }
}
