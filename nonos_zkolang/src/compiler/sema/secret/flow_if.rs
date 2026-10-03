/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `if` (sections 8.4 and 13.1). Each condition runs under the conditions before it, and
 * each branch under those and its own, so what either assigns takes that guard's label,
 * and so does the value of the `if`. A branch that leaves the function or a loop leaves
 * the rest of it under the guard too.
 */

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::TyId;
use crate::compiler::tir::{TBlock, TExpr};

impl<'p> Flow<'p> {
    /** The labels of the value of an `if`. */
    pub(super) fn if_(
        &mut self,
        branches: &[(TExpr, TBlock)],
        last: Option<&TBlock>,
        ty: TyId,
    ) -> Shape {
        let (pc0, mut before) = (self.pc, self.env.clone());
        let (ret0, exit0) = (self.ret_guard, self.exits.last().copied());
        let (mut guard, mut inner) = (Taint::PUBLIC, Taint::PUBLIC);
        let mut merged: Option<alloc::vec::Vec<Shape>> = None;
        let mut value: Option<Shape> = None;
        let arms = branches.iter().map(|(c, b)| (Some(c), Some(b)));
        for (cond, block) in arms.chain(core::iter::once((None, last))) {
            self.env = before.clone();
            self.pc = pc0.join(guard);
            if let Some(c) = cond {
                guard = guard.join(self.expr(c).all());
                before = self.env.clone();
            }
            self.pc = pc0.join(guard);
            let v = match block {
                Some(b) => self.block(b),
                None => Shape::of(ty, Taint::PUBLIC, &self.program.types),
            };
            inner = inner.join(self.pc);
            value = Some(value.map_or(v.clone(), |x| x.join(&v)));
            let env = core::mem::take(&mut self.env);
            merged = Some(match merged {
                Some(m) => m.iter().zip(&env).map(|(a, b)| a.join(b)).collect(),
                None => env,
            });
        }
        self.env = merged.unwrap_or(before);
        let left = self.ret_guard != ret0 || self.exits.last().copied() != exit0;
        self.pc = if left { pc0.join(inner) } else { pc0 };
        value.unwrap_or(Shape::Leaf(Taint::PUBLIC)).raised(guard)
    }
}
