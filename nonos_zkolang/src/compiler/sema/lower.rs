/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Types as written, lowered to types of the table (section 5), with the labels written on
 * them recorded apart (section 5.4). An alias is replaced by what it stands for.
 */

use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::ast::{Type, TypeKind};
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /** The type `t`, written in module `m`, and the labels it writes. */
    pub(crate) fn lower_ty(&mut self, m: DefId, t: &'a Type) -> (TyId, Labels) {
        let mut labels = Labels::default();
        let ty = self.lower_at(m, t, &mut Vec::new(), &mut labels);
        (ty, labels)
    }

    /** The type `t` at `path` in the whole, its labels added to `labels`. */
    fn lower_at(
        &mut self,
        m: DefId,
        t: &'a Type,
        path: &mut Vec<u32>,
        labels: &mut Labels,
    ) -> TyId {
        match &t.kind {
            TypeKind::Field => Types::FIELD,
            TypeKind::Bool => Types::BOOL,
            TypeKind::Int(i) => Types::int(*i),
            TypeKind::Unit => Types::UNIT,
            TypeKind::Tuple(ts) => {
                let mut elems = Vec::with_capacity(ts.len());
                for (i, e) in ts.iter().enumerate() {
                    path.push(u32::try_from(i).unwrap_or(u32::MAX));
                    elems.push(self.lower_at(m, e, path, labels));
                    path.pop();
                }
                self.types.intern(TyKind::Tuple(elems))
            }
            TypeKind::Array(e, n) => {
                path.push(Labels::ELEMENT);
                let el = self.lower_at(m, e, path, labels);
                path.pop();
                match self.const_usize(m, n) {
                    Some(n) => self.types.intern(TyKind::Array(el, n)),
                    None => Types::ERROR,
                }
            }
            TypeKind::Labelled(l, inner) => {
                labels.0.push((path.clone(), *l));
                self.lower_at(m, inner, path, labels)
            }
            TypeKind::RefMut(inner) => self.lower_at(m, inner, path, labels),
            TypeKind::Path(p) => self.lower_path(m, p, path, labels),
            TypeKind::SelfType => {
                let (ty, own) = self.self_type(t.span);
                for (q, l) in own.0 {
                    labels.0.push(([&path[..], &q].concat(), l));
                }
                ty
            }
            TypeKind::Error => Types::ERROR,
        }
    }
}
