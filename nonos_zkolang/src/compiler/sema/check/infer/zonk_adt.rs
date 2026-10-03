/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Resolving the instances inference leaves open: an instance of a generic struct or enum
 * whose arguments hold variables becomes the instance for what they stand for. A general
 * variable nothing settles is reported where it arose (E0303).
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{GenArg, TyId};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The instance `t` with its arguments resolved. */
    pub(super) fn zonk_adt(&mut self, t: TyId, default: bool) -> TyId {
        let Some((def, args)) = self.sema.types.adt(t).map(|a| (a.def, a.args.clone())) else {
            return t;
        };
        let resolved: Vec<GenArg> = args
            .iter()
            .map(|g| match *g {
                GenArg::Type(e) => GenArg::Type(self.zonk(e, default)),
                c => c,
            })
            .collect();
        if resolved == args {
            return t;
        }
        self.sema.adt_ty(DefId(def), &resolved).0
    }

    /** Report the general variable `n`, which nothing settles (E0303). */
    pub(super) fn cannot_infer(&mut self, n: u32) {
        let Some((at, what)) = self.vars.origin(n) else {
            return;
        };
        let d = Diagnostic::error(
            Code::CANNOT_INFER,
            format!("cannot infer {what}"),
            at,
            "a type is needed here",
        )
        .with_help("write it: annotate the binding, or give the generic arguments with `::<..>`");
        self.sema.diags.push(d);
    }
}
