/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Binding the names of a struct or variant pattern, each field at its index. */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::super::labels::sub_labels;
use super::pat_struct::Part;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::Pattern;
use crate::compiler::tir::{Labels, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * Bind the names of the struct or variant pattern `p`, which takes a value of type
     * `ty`. A variant of an enum of more than one may not match, so it stands only in a
     * `match` arm.
     */
    pub(crate) fn bind_struct(
        &mut self,
        p: &'a Pattern,
        ty: TyId,
        labels: &Labels,
        path: &mut Vec<u32>,
    ) -> TPat {
        let Some((s, parts)) = self.struct_parts(p, ty) else {
            return TPat::Wild;
        };
        let types: Vec<TyId> = self.shape_fields(s).into_iter().map(|f| f.1).collect();
        let mut out = Vec::with_capacity(parts.len());
        for (i, part) in parts.into_iter().enumerate() {
            let t = types.get(i).copied().unwrap_or(Types::ERROR);
            path.push(u32::try_from(i).unwrap_or(u32::MAX));
            out.push(match part {
                Part::Pat(q) => self.pat(q, t, labels, path),
                Part::Name(n) => {
                    let own = sub_labels(labels, path);
                    TPat::Bind(self.pat_bind(n, t, false, own))
                }
                Part::Ignored => TPat::Wild,
            });
            path.pop();
        }
        let variants = self.sema.types.adt(s.0).filter(|a| a.is_enum);
        match variants.map(|a| a.variants.len()) {
            None => TPat::Tuple(out),
            Some(n) => {
                if n > 1 && !self.pats.arm {
                    self.refutable(p);
                }
                TPat::Variant(s.1, out)
            }
        }
    }
}
