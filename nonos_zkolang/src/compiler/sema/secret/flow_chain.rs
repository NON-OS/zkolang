/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Operator chains. The right operand of `&&` or `||` runs only when the operands before
 * it do not decide the result (section 7.4), so it runs under their guard: what it
 * assigns takes their labels, and keeps the labels it had when it does not run.
 */

use super::flow::Flow;
use super::shape::Shape;
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::tir::TExpr;

impl<'p> Flow<'p> {
    /** The labels of `first op1 e1 op2 e2 ...`. */
    pub(super) fn chain(&mut self, first: &TExpr, links: &[(BinOp, TExpr)]) -> Shape {
        let mut t = self.expr(first).all();
        for (op, e) in links {
            if !matches!(op, BinOp::And | BinOp::Or) {
                t = t.join(self.expr(e).all());
                continue;
            }
            let (pc0, env0) = (self.pc, self.env.clone());
            let (ret0, exit0) = (self.ret_guard, self.exits.last().copied());
            self.pc = pc0.join(t);
            t = t.join(self.expr(e).all());
            let inner = self.pc;
            self.env = env0.iter().zip(&self.env).map(|(a, b)| a.join(b)).collect();
            let left = self.ret_guard != ret0 || self.exits.last().copied() != exit0;
            self.pc = if left { pc0.join(inner) } else { pc0 };
        }
        Shape::Leaf(t)
    }
}
