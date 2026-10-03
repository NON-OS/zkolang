/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Blocks and statements (section 8): `let` binds the slots of its value to the pattern's
 * locals, `assert` fails the run where it runs and its condition is false, and a block's
 * value is its tail's.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::L;
use crate::compiler::ssa::V;
use crate::compiler::tir::{TBlock, TStmt};

impl<'p> Lower<'p> {
    /** The value of the block `b`. */
    pub(super) fn block(&mut self, b: &TBlock) -> L<Vec<V>> {
        for s in &b.stmts {
            match s {
                TStmt::Let { pat, init } => {
                    let v = self.expr(init)?;
                    self.bind(pat, &v, init.ty)?;
                }
                TStmt::Assert { cond, .. } => {
                    let outer = self.enter(cond.span);
                    let c = self.expr(cond)?;
                    if let Some(&c) = c.first() {
                        self.require(c);
                    }
                    self.b.site = outer;
                }
                TStmt::Expr(e) => {
                    self.expr(e)?;
                }
            }
        }
        match &b.tail {
            Some(t) => self.expr(t),
            None => Ok(Vec::new()),
        }
    }
}
