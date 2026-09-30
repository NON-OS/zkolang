/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Binding a pattern's locals to the labels of the parts of a value they take. */

use super::flow::Flow;
use super::parts::variant_part;
use super::shape::Shape;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::tir::TPat;

impl<'p> Flow<'p> {
    /** Bind the locals of `pat` to parts of a value of type `ty`, labelled `v`, from `at`. */
    pub(super) fn bind(&mut self, pat: &TPat, v: Shape, ty: TyId, at: Span) {
        match pat {
            TPat::Bind(l) => {
                let Some(local) = self.f.locals.get(l.0 as usize) else {
                    return;
                };
                let v = self.labelled(v, &local.labels.0, local.ty, at);
                if let Some(slot) = self.env.get_mut(l.0 as usize) {
                    *slot = v;
                }
            }
            TPat::Wild => {}
            TPat::Tuple(ps) => {
                let kind = self.program.types.kind(ty).clone();
                let record = self.program.types.record(ty);
                for (i, p) in ps.iter().enumerate() {
                    let (part, part_ty) = match (&record, &kind) {
                        (Some(ts), _) => (v.child(i), ts.get(i).copied().unwrap_or(ty)),
                        (None, TyKind::Array(el, _)) => (v.elem(), *el),
                        _ => (v.clone(), ty),
                    };
                    self.bind(p, part, part_ty, at);
                }
            }
            TPat::Variant(tag, ps) => {
                let tys = self.program.types.parts(ty, Some(*tag));
                for (i, (p, f)) in ps.iter().zip(tys).enumerate() {
                    let k = variant_part(&self.program.types, ty, *tag, i as u32);
                    self.bind(p, v.child(k), f, at);
                }
            }
            TPat::Or(alts) => alts.iter().for_each(|a| self.bind(a, v.clone(), ty, at)),
            TPat::Lit(..) | TPat::Range(..) => {}
        }
    }
}
