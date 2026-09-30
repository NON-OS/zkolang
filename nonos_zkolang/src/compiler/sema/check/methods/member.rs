/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The function a type gives a name (section 10.2): one of an `impl` block for exactly that
 * type, or else one of a generic `impl` block for its struct or enum, with the arguments
 * the type gives the block. It is visible where its block's module is, or everywhere if
 * `pub`.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{GenArg, TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Visibility;
use crate::compiler::tir::FnId;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The function `name` of the type `ty`, visible here, and the arguments `ty` gives its
     * generic `impl` block, if it has one; `None` once reported.
     */
    pub(crate) fn member_fn(
        &mut self,
        ty: TyId,
        name: &str,
        at: Span,
    ) -> Option<(FnId, Vec<GenArg>)> {
        if ty == Types::ERROR {
            return None;
        }
        let ty = self.zonk(ty, false);
        let exact = self.sema.assoc.get(&(ty, String::from(name))).copied();
        let def = self.sema.types.adt(ty).map(|a| DefId(a.def));
        let generic = def.and_then(|d| {
            self.sema
                .generic_assoc
                .get(&(d, String::from(name)))
                .copied()
        });
        let found = match (exact, generic) {
            (Some(f), _) => Some((f, Vec::new())),
            (None, Some(t)) => self.impl_args(t, ty).map(|a| (t, a)),
            (None, None) => None,
        };
        let Some((fid, args)) = found else {
            let what = match self.sema.types.adt(ty).is_some_and(|a| a.is_enum) {
                true => format!("`{}` has no variant or function `{name}`", self.show(ty)),
                false => format!("`{}` has no function `{name}`", self.show(ty)),
            };
            let d = Diagnostic::error(Code::NO_FIELD, what, at, "not found");
            self.sema.diags.push(d);
            return None;
        };
        let info = self.sema.fns.get(fid.0 as usize).map(|f| (f.def, f.module));
        let vis = info.and_then(|(d, _)| self.sema.defs.get(d)).map(|d| d.vis);
        if let (Some((_, m)), Some(Visibility::Private)) = (info, vis) {
            if !self.sema.defs.within(self.module, m) {
                let what = format!("the function `{name}` of `{}` is private", self.show(ty));
                let d = Diagnostic::error(Code::PRIVATE_ITEM, what, at, "private");
                self.sema
                    .diags
                    .push(d.with_help("mark it `pub` in its `impl` block"));
            }
        }
        Some((fid, args))
    }
}
