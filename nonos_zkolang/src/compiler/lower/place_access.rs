/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading and writing the slots a place's evaluated steps name. */

use alloc::vec::Vec;

use super::cx::Lower;
use super::layout::{elements, field_at, slots};
use super::place::Step;
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::tir::LocalId;

impl<'p> Lower<'p> {
    /** The slots the steps name in local `root`. */
    pub(super) fn read_steps(&mut self, root: LocalId, steps: &[Step]) -> Vec<V> {
        let (vals, ty) = (self.local(root), self.local_ty(root));
        self.read_at(&vals, ty, steps)
    }

    /** The slots the steps name in `vals`, a value of type `ty`. */
    pub(super) fn read_at(&mut self, vals: &[V], ty: TyId, steps: &[Step]) -> Vec<V> {
        let types = &self.p.types;
        let Some((&step, rest)) = steps.split_first() else {
            return vals.to_vec();
        };
        let (at, size, t, dynamic) = match (step, field_at(types, ty, 0), elements(types, ty)) {
            (Step::Field(i), _, _) => {
                let (at, t) = field_at(types, ty, i).unwrap_or((0, ty));
                (at, slots(types, t), t, None)
            }
            (Step::At(k), _, Some((e, size, _))) => (k * size, size, e, None),
            (Step::Dyn(i), _, Some((e, size, n))) => (0, size, e, Some((i, n))),
            _ => return Vec::new(),
        };
        let Some((i, n)) = dynamic else {
            return self.read_at(vals.get(at..at + size).unwrap_or(&[]), t, rest);
        };
        let zero = self.b.konst(0);
        let mut out = alloc::vec![zero; size];
        for k in 0..n {
            let kv = self.b.konst(k as i128);
            let is = self.b.emit(Inst::Eq(i, kv));
            let elem = vals.get(k * size..(k + 1) * size).unwrap_or(&[]);
            for (o, &x) in out.iter_mut().zip(elem) {
                *o = self.b.sel(is, x, *o);
            }
        }
        self.read_at(&out, t, rest)
    }

    /** Assign `new` to the slots the steps name in local `root`, where the point runs. */
    pub(super) fn write_steps(&mut self, root: LocalId, steps: &[Step], new: &[V]) {
        let (mut vals, ty, g) = (self.local(root), self.local_ty(root), self.g);
        self.write_at(&mut vals, 0, ty, steps, new, g);
        self.set_local(root, vals);
    }
}
