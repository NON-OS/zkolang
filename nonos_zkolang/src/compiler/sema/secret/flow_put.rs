/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Replacing the labels of the part of a value that a place's projections name. */

use alloc::boxed::Box;

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::tir::Proj;

impl<'p> Flow<'p> {
    /** `old`, of type `ty`, with the part at `proj` set to `v`, raised by `index` there. */
    pub(super) fn put(&self, old: Shape, ty: TyId, proj: &[Proj], v: Shape, index: Taint) -> Shape {
        let Some((step, rest)) = proj.split_first() else {
            return v;
        };
        let types = &self.program.types;
        let old = match old {
            Shape::Leaf(t) => Shape::of(ty, t, types),
            s => s,
        };
        match (step, types.kind(ty), old) {
            (Proj::TupleField(i), TyKind::Tuple(ts), Shape::Tuple(mut parts)) => {
                let i = *i as usize;
                if let (Some(p), Some(&t)) = (parts.get_mut(i), ts.get(i)) {
                    let cur = core::mem::replace(p, Shape::Leaf(Taint::PUBLIC));
                    *p = self.put(cur, t, rest, v, index);
                }
                Shape::Tuple(parts)
            }
            (Proj::Index(_), TyKind::Array(el, _), Shape::Array(e)) => {
                let written = self.put((*e).clone(), *el, rest, v, index);
                Shape::Array(Box::new(e.join(&written).raised(index)))
            }
            (_, _, old) => Shape::Leaf(old.all().join(v.all()).join(index)),
        }
    }
}
