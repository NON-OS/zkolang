/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `match` (section 8.5), converted like `if` (section 21.4): each arm runs under the guard
 * that the arms before it were not taken, its pattern matches and its guard holds; its
 * locals are bound from the scrutinee's slots; the value is merged with `Sel`.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::cx::Lower;
use super::error::L;
use super::layout::slots;
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::V;
use crate::compiler::tir::{TArm, TExpr};

impl<'p> Lower<'p> {
    /** The value of `match scrut { arms }`, of type `t`. */
    pub(super) fn match_(&mut self, scrut: &TExpr, arms: &[TArm], t: TyId) -> L<Vec<V>> {
        let v = self.expr(scrut)?;
        let zero = self.b.konst(0);
        let mut value = vec![zero; slots(&self.p.types, t)];
        let mut after = zero;
        for arm in arms {
            let c = self.pat_cond(&arm.pat, &v, scrut.ty)?;
            let rest = self.g;
            let matched = self.and(rest, c);
            self.g = matched;
            self.bind(&arm.pat, &v, scrut.ty)?;
            if let Some(guard) = &arm.guard {
                let holds = self.expr(guard)?.first().copied().unwrap_or(zero);
                self.g = self.and(self.g, holds);
            }
            let taken = self.g;
            let body = self.expr(&arm.body)?;
            value = self.pick(taken, &body, &value);
            after = self.b.add(after, self.g);
            self.g = self.b.sub(rest, taken);
        }
        self.g = self.b.add(after, self.g);
        Ok(value)
    }
}
