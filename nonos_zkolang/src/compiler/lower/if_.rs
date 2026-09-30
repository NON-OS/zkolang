/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `if` (section 8.4). Both arms are lowered, one after the other, each under the guard of
 * the conditions before it and its own; the value is selected by the arm's guard. The guard
 * after the `if` is the sum of the arms' guards at their ends: the paths that did not
 * leave in an arm.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::cx::Lower;
use super::error::L;
use super::layout::slots;
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::V;
use crate::compiler::tir::{TBlock, TExpr};

impl<'p> Lower<'p> {
    /** The value of `if c1 { .. } else if c2 { .. } else { .. }`, of type `t`. */
    pub(super) fn if_(
        &mut self,
        branches: &[(TExpr, TBlock)],
        last: Option<&TBlock>,
        t: TyId,
    ) -> L<Vec<V>> {
        let n = slots(&self.p.types, t);
        let zero = self.b.konst(0);
        let mut value = vec![zero; n];
        let mut after = zero;
        for (cond, block) in branches {
            let c = self.expr(cond)?.first().copied().unwrap_or(zero);
            let rest = self.g;
            let arm = self.and(rest, c);
            self.g = arm;
            let v = self.block(block)?;
            value = self.pick(arm, &v, &value);
            after = self.b.add(after, self.g);
            self.g = self.b.sub(rest, arm);
        }
        let rest = self.g;
        if let Some(block) = last {
            let v = self.block(block)?;
            value = self.pick(rest, &v, &value);
        }
        self.g = self.b.add(after, self.g);
        Ok(value)
    }

    /** The slots of `v` where `c`, else `old`; a value with no slots, from an arm that
     * leaves, keeps `old`. */
    pub(super) fn pick(&mut self, c: V, v: &[V], old: &[V]) -> Vec<V> {
        if v.len() != old.len() {
            return old.to_vec();
        }
        v.iter()
            .zip(old)
            .map(|(&a, &b)| self.b.sel(c, a, b))
            .collect()
    }
}
