/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Writing through a place's steps, each element of a run-time index under its equality. */

use super::cx::Lower;
use super::layout::{elements, field_at};
use super::place::Step;
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /** Set the part of `vals` at `at`, of type `ty`, that `steps` name to `new` where `cond`. */
    pub(super) fn write_at(
        &mut self,
        vals: &mut [V],
        at: usize,
        ty: TyId,
        steps: &[Step],
        new: &[V],
        cond: V,
    ) {
        let Some((&step, rest)) = steps.split_first() else {
            for (k, &n) in new.iter().enumerate() {
                if let Some(slot) = vals.get_mut(at + k) {
                    *slot = self.b.sel(cond, n, *slot);
                }
            }
            return;
        };
        let types = &self.p.types;
        match (step, elements(types, ty)) {
            (Step::Field(i), _) => {
                if let Some((off, t)) = field_at(types, ty, i) {
                    self.write_at(vals, at + off, t, rest, new, cond);
                }
            }
            (Step::At(k), Some((e, size, _))) => {
                self.write_at(vals, at + k * size, e, rest, new, cond)
            }
            (Step::Dyn(i), Some((e, size, n))) => {
                for k in 0..n {
                    let kv = self.b.konst(k as i128);
                    let is = self.b.emit(Inst::Eq(i, kv));
                    let c = self.and(cond, is);
                    self.write_at(vals, at + k * size, e, rest, new, c);
                }
            }
            _ => {}
        }
    }
}
