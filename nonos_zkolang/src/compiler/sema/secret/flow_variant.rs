/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Enum values and patterns in secret flow (section 13): a variant built where it is has
 * a public tag and the labels of its fields, each meeting its field's labels; a pattern
 * tests the tag of each variant it names and the parts its literals and ranges compare.
 */

use super::flow::Flow;
use super::parts::variant_part;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::TyId;
use crate::compiler::tir::TExpr;

impl<'p> Flow<'p> {
    /** The labels of the variant `tag` of the enum `ty`, built from `fs`. */
    pub(super) fn variant(&mut self, tag: u32, fs: &[(u32, TExpr)], ty: TyId) -> Shape {
        let Shape::Tuple(mut parts) = Shape::of(ty, Taint::PUBLIC, &self.program.types) else {
            return Shape::Leaf(Taint::PUBLIC);
        };
        let adt = self.program.types.adt(ty);
        let fields = adt
            .and_then(|a| a.variants.get(tag as usize))
            .map(|v| v.fields.clone());
        let fields = fields.unwrap_or_default();
        for (i, x) in fs {
            let v = self.expr(x);
            let v = match fields.get(*i as usize) {
                Some(f) => self.labelled(v, &f.labels.0, f.ty, x.span),
                None => v,
            };
            let k = variant_part(&self.program.types, ty, tag, *i);
            if let Some(p) = parts.get_mut(k) {
                *p = v;
            }
        }
        Shape::Tuple(parts)
    }
}
