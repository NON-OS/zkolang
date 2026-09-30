/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A struct literal (section 13.2): each field is a checked position, its value meeting the
 * labels the field's type writes, in the order the fields are written.
 */

use alloc::vec;

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::TyId;
use crate::compiler::tir::TExpr;

impl<'p> Flow<'p> {
    /** The labels of the struct literal `fs` of type `ty`. */
    pub(super) fn record(&mut self, fs: &[(u32, TExpr)], ty: TyId) -> Shape {
        let fields = self
            .program
            .types
            .adt(ty)
            .and_then(|a| a.variants.first())
            .map(|v| v.fields.clone())
            .unwrap_or_default();
        let mut parts = vec![Shape::Leaf(Taint::PUBLIC); fields.len()];
        for (i, x) in fs {
            let v = self.expr(x);
            let Some(f) = fields.get(*i as usize) else {
                continue;
            };
            let v = self.labelled(v, &f.labels.0, f.ty, x.span);
            if let Some(p) = parts.get_mut(*i as usize) {
                *p = v;
            }
        }
        Shape::Tuple(parts)
    }
}
