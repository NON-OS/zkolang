/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Assignment (section 8.2) and indexing. `place op= v` reads the place and writes it once,
 * through the same evaluated steps; `a[i]` on a value reads the element `i` names.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use super::place::Step;
use crate::compiler::ssa::V;
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::tir::{TExpr, TExprKind, TLit, TPlace};

impl<'p> Lower<'p> {
    /** `place = value`, or `place op= value`. */
    pub(super) fn assign_expr(
        &mut self,
        place: &TPlace,
        op: Option<BinOp>,
        value: &TExpr,
    ) -> L<()> {
        let steps = self.steps(place)?;
        let mut v = self.expr(value)?;
        if let Some(op) = op {
            let old = self.read_steps(place.root, &steps);
            v = self.binop(op, &old, &v, place.ty, place.span)?;
        }
        self.write_steps(place.root, &steps, &v);
        Ok(())
    }

    /** `a[i]`. */
    pub(super) fn index(&mut self, a: &TExpr, i: &TExpr) -> L<Vec<V>> {
        let vals = self.expr(a)?;
        let Some((_, _, n)) = super::layout::elements(&self.p.types, a.ty) else {
            return Err(LowerError::Unsupported("an index into this value", a.span));
        };
        let step = match i.kind {
            TExprKind::Lit(TLit::Int(k)) => Step::At(k as usize),
            _ => {
                let iv = self
                    .expr(i)?
                    .first()
                    .copied()
                    .ok_or(LowerError::Unsupported("an index", i.span))?;
                self.check_index(iv, n);
                Step::Dyn(iv)
            }
        };
        Ok(self.read_at(&vals, a.ty, &[step]))
    }
}
