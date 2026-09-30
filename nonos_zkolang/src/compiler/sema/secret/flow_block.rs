/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Blocks and the locals `let` binds. A local declared with labels checks its `public`
 * parts and raises its `secret` ones (section 13.2).
 */

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::tir::{TBlock, TPat, TStmt};

impl<'p> Flow<'p> {
    /** The labels of a block's value, after its statements. */
    pub(super) fn block(&mut self, b: &TBlock) -> Shape {
        for s in &b.stmts {
            match s {
                TStmt::Let { pat, init } => {
                    let v = self.expr(init);
                    self.bind(pat, v, init.ty, init.span);
                }
                TStmt::Assert { cond, .. } => {
                    self.expr(cond);
                }
                TStmt::Expr(e) => {
                    self.expr(e);
                }
            }
        }
        b.tail
            .as_ref()
            .map_or(Shape::Leaf(Taint::PUBLIC), |t| self.expr(t))
    }

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
                for (i, p) in ps.iter().enumerate() {
                    let (part, part_ty) = match &kind {
                        TyKind::Tuple(ts) => (v.child(i), ts.get(i).copied().unwrap_or(ty)),
                        TyKind::Array(el, _) => (v.elem(), *el),
                        _ => (v.clone(), ty),
                    };
                    self.bind(p, part, part_ty, at);
                }
            }
        }
    }
}
