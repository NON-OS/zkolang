/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The generic arguments of a call of an `impl` block's function: the block's, which the
 * type gives, then the function's own, written with `::<..>` or else inferred. A function
 * that takes none refuses them written.
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::GenericArg;
use crate::compiler::tir::FnId;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * For the function `fid`, given `impl_args` by its type: its template's arguments, or
     * `None` inside `Some` if it is no template; `None` once reported.
     */
    pub(crate) fn member_generics(
        &mut self,
        (fid, impl_args): (FnId, Vec<GenArg>),
        given: Option<&'a [GenericArg]>,
        at: Span,
    ) -> Option<Option<Vec<GenArg>>> {
        let info = self.sema.fns.get(fid.0 as usize)?;
        if !info.template {
            if given.is_some() {
                let what = format!("`{}` takes no generic arguments", info.decl.name.name);
                let d =
                    Diagnostic::error(Code::WRONG_GENERICS, what, at, "generic arguments given");
                self.sema.diags.push(d);
            }
            return Some(None);
        }
        let own = self.generic_args(info.def, given, at)?;
        Some(Some(impl_args.into_iter().chain(own).collect()))
    }
}
