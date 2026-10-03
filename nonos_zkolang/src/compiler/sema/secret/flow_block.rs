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
use crate::compiler::tir::{TBlock, TStmt};

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
}
