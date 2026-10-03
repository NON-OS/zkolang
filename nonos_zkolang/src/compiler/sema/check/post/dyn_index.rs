/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The dynamic index warning (W0101, section 15.3): a runtime index, read or written, into an array longer than the threshold, in the program's own crate. */

use alloc::format;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::tir::{Proj, TExpr, TPlace};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The index `i` into a value of type `ty`, which warns if it is runtime and the array long. */
    pub(super) fn post_dyn(&mut self, ty: TyId, i: &TExpr) {
        let TyKind::Array(_, n) = self.kind(ty) else {
            return;
        };
        let limit = self.sema.limits.dyn_index_warn;
        if u64::from(n) <= limit || self.sema.not_const(i).is_none() || !self.own() {
            return;
        }
        let what = format!("a runtime index into an array of {n} elements, more than {limit}");
        let d = Diagnostic::warning(Code::COST_DYNAMIC_INDEX, what, i.span, "not a constant")
            .with_help("each element is rows; set `dyn_index_warn` in the manifest's `[cost]`, or put the attribute `allow(cost)` on the function");
        self.sema.diags.push(d);
    }

    /** The place `p` written: each runtime index on its way. */
    pub(super) fn post_place(&mut self, p: &TPlace) {
        let mut ty = self
            .locals
            .get(p.root.0 as usize)
            .map_or(Types::ERROR, |l| l.ty);
        for step in &p.proj {
            let next = match step {
                Proj::Index(i) => {
                    self.post_dyn(ty, i);
                    match self.kind(ty) {
                        TyKind::Array(e, _) => Some(e),
                        _ => None,
                    }
                }
                Proj::TupleField(k) => self
                    .sema
                    .types
                    .record(ty)
                    .and_then(|r| r.get(*k as usize).copied()),
            };
            let Some(next) = next else {
                return;
            };
            ty = next;
        }
    }
}
