/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What a pattern tests: the tag of each variant it names and the parts its literals and ranges compare; a binding tests nothing. */

use alloc::vec::Vec;

use super::flow::Flow;
use super::parts::variant_part;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::tir::TPat;

impl<'p> Flow<'p> {
    /** The labels of the parts of `v`, of type `ty`, that the pattern `p` tests. */
    pub(super) fn tested(&self, p: &TPat, v: &Shape, ty: TyId) -> Taint {
        let types = &self.program.types;
        match p {
            TPat::Bind(_) | TPat::Wild => Taint::PUBLIC,
            TPat::Lit(..) | TPat::Range(..) => v.all(),
            TPat::Or(alts) => alts
                .iter()
                .fold(Taint::PUBLIC, |t, a| t.join(self.tested(a, v, ty))),
            TPat::Tuple(ps) => {
                let tys = types.parts(ty, None);
                let array = matches!(types.kind(ty), TyKind::Array(..));
                let each = ps.iter().zip(tys).enumerate();
                let parts: Vec<Taint> = each
                    .map(|(i, (q, t))| {
                        self.tested(q, &if array { v.elem() } else { v.child(i) }, t)
                    })
                    .collect();
                parts.into_iter().fold(Taint::PUBLIC, Taint::join)
            }
            TPat::Variant(tag, ps) => {
                let tys = types.parts(ty, Some(*tag));
                let each = ps.iter().zip(tys).enumerate();
                each.fold(v.child(0).all(), |t, (i, (q, f))| {
                    let k = variant_part(types, ty, *tag, i as u32);
                    t.join(self.tested(q, &v.child(k), f))
                })
            }
        }
    }
}
