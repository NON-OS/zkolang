/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `break`, `continue` and `return` (section 8.8). What runs after one runs under its
 * guard: the rest of the loop's iterations, or the rest of the function. A `return` leaves
 * every open loop, and the value each `&mut` parameter has there is one it may end with.
 */

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::tir::TExpr;

impl<'p> Flow<'p> {
    /** `break` or `continue`: the rest of the loop runs under the current guard. */
    pub(super) fn leave_loop(&mut self) -> Shape {
        let pc = self.pc;
        if let Some(x) = self.exits.last_mut() {
            *x = x.join(pc);
        }
        Shape::Leaf(Taint::PUBLIC)
    }

    /** `return v`. */
    pub(super) fn return_(&mut self, v: Option<&TExpr>) -> Shape {
        let s = v.map_or(Shape::Leaf(Taint::PUBLIC), |v| self.expr(v));
        let pc = self.pc;
        self.returned = self.returned.join(&s.raised(pc));
        self.ret_guard = self.ret_guard.join(pc);
        for x in &mut self.exits {
            *x = x.join(pc);
        }
        for i in 0..self.ref_ret.len() {
            let Some((local, _)) = self.ref_ret.get(i) else {
                continue;
            };
            let now = self
                .env
                .get(*local)
                .cloned()
                .unwrap_or(Shape::Leaf(Taint::SECRET));
            if let Some((_, seen)) = self.ref_ret.get_mut(i) {
                *seen = seen.join(&now);
            }
        }
        Shape::Leaf(Taint::PUBLIC)
    }
}
