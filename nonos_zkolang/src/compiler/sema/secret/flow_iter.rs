/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One pass over a loop's body, under the guards of the loop's exits so far. */

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'p> Flow<'p> {
    /** One pass over the loop `e`'s body; the label of its condition. */
    pub(super) fn iteration(&mut self, e: &TExpr, pc0: Taint) -> Taint {
        let guard = pc0.join(self.exits.last().copied().unwrap_or(Taint::SECRET));
        self.pc = guard;
        match &e.kind {
            TExprKind::ForRange { var, body, .. } => {
                if let Some(slot) = self.env.get_mut(var.0 as usize) {
                    *slot = Shape::Leaf(Taint::PUBLIC);
                }
                self.block(body);
                Taint::PUBLIC
            }
            TExprKind::ForArray {
                index,
                pat,
                array,
                body,
            } => {
                let items = self.expr(array);
                if let Some(slot) = index.and_then(|i| self.env.get_mut(i.0 as usize)) {
                    *slot = Shape::Leaf(Taint::PUBLIC);
                }
                let el = match self.program.types.kind(array.ty) {
                    TyKind::Array(el, _) => *el,
                    _ => array.ty,
                };
                self.bind(pat, items.elem(), el, array.span);
                self.block(body);
                Taint::PUBLIC
            }
            TExprKind::While { cond, body, .. } => {
                let c = self.expr(cond).all();
                self.pc = guard.join(c);
                self.block(body);
                c
            }
            _ => Taint::PUBLIC,
        }
    }
}
