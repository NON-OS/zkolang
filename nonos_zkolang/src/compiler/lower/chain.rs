/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Operator chains, left to right (section 7). The right operand of `&&` or `||` runs
 * only when the left does not decide the result (section 7.4): it is lowered under the
 * guard of the left's value, so its failures only count where it runs.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::L;
use crate::compiler::ssa::V;
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::tir::TExpr;

impl<'p> Lower<'p> {
    /** `first op1 e1 op2 e2 ...`. */
    pub(super) fn chain(&mut self, first: &TExpr, links: &[(BinOp, TExpr)]) -> L<Vec<V>> {
        let mut acc = self.expr(first)?;
        for (op, e) in links {
            acc = match op {
                BinOp::And | BinOp::Or => {
                    let a = acc.first().copied().unwrap_or(self.g);
                    let b = self.short_circuit(*op == BinOp::And, a, e)?;
                    alloc::vec![b]
                }
                _ => {
                    let rhs = self.expr(e)?;
                    self.binop(*op, &acc, &rhs, first.ty, e.span)?
                }
            };
        }
        Ok(acc)
    }

    /** `a && e` when `and`, else `a || e`, `e` lowered where it runs. */
    fn short_circuit(&mut self, and: bool, a: V, e: &TExpr) -> L<V> {
        let g0 = self.g;
        let na = self.b.not(a);
        let decides = if and { na } else { a };
        let skip = self.and(g0, decides);
        self.g = self.b.sub(g0, skip);
        let b = self.expr(e)?.first().copied().unwrap_or(g0);
        /* The paths that skipped the operand, and those that came through it. */
        self.g = self.b.add(skip, self.g);
        let ab = self.and(a, b);
        Ok(match and {
            true => ab,
            false => {
                let s = self.b.add(a, b);
                self.b.sub(s, ab)
            }
        })
    }
}
