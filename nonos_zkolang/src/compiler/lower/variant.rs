/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Enum values (section 6): the tag, the variant's fields at the start of the payload, and
 * every other payload slot 0. The fields are evaluated in the order written.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::cx::Lower;
use super::error::L;
use super::layout::{slots, variant_field_at};
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::V;
use crate::compiler::tir::TExpr;

impl<'p> Lower<'p> {
    /** The slots of the variant `tag` of the enum `t`, built from `fs`. */
    pub(super) fn variant(&mut self, tag: u32, fs: &[(u32, TExpr)], t: TyId) -> L<Vec<V>> {
        let zero = self.b.konst(0);
        let mut out = vec![zero; slots(&self.p.types, t)];
        if let Some(first) = out.first_mut() {
            *first = self.b.konst(i128::from(tag));
        }
        for (i, x) in fs {
            let v = self.expr(x)?;
            let Some((at, _)) = variant_field_at(&self.p.types, t, tag, *i) else {
                continue;
            };
            for (k, s) in v.into_iter().enumerate() {
                if let Some(slot) = out.get_mut(at + k) {
                    *slot = s;
                }
            }
        }
        Ok(out)
    }
}
