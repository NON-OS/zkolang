/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `return`, `break` and `continue` (section 8.8). Each records the guard it runs under,
 * `return` selecting its value into the call's result, and sets the guard to 0: nothing
 * after it runs on this path, until a loop's iteration or the call ends.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::L;
use crate::compiler::ssa::V;
use crate::compiler::tir::TExpr;

impl<'p> Lower<'p> {
    /** `return v`. */
    pub(super) fn return_(&mut self, v: Option<&TExpr>) -> L<Vec<V>> {
        let vals = match v {
            Some(v) => self.expr(v)?,
            None => Vec::new(),
        };
        let g = self.g;
        if let Some(ret) = self.frames.last().map(|f| f.ret.clone()) {
            let new: Vec<V> = match vals.len() == ret.len() {
                true => vals
                    .iter()
                    .zip(&ret)
                    .map(|(&a, &b)| self.b.sel(g, a, b))
                    .collect(),
                false => ret,
            };
            if let Some(f) = self.frame() {
                f.ret = new;
            }
        }
        self.g = self.b.konst(0);
        Ok(Vec::new())
    }

    /** `break` when `brk`, else `continue`. */
    pub(super) fn exit_loop(&mut self, brk: bool) -> Vec<V> {
        let g = self.g;
        if let Some(lp) = self.innermost().copied() {
            let new = match brk {
                true => self.b.add(lp.broke, g),
                false => self.b.add(lp.cont, g),
            };
            if let Some(l) = self.innermost() {
                if brk {
                    l.broke = new;
                } else {
                    l.cont = new;
                }
            }
        }
        self.g = self.b.konst(0);
        Vec::new()
    }
}
