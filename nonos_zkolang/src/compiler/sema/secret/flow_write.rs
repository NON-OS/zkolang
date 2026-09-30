/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Assignment: the part a place names takes the value's labels and the guard's. A write
 * through an index may change any element, so every element takes the index's label.
 * The variable written keeps the labels its type was written with (section 13.2).
 */

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::Types;
use crate::compiler::source::Span;
use crate::compiler::tir::{Proj, TExpr, TPlace};

impl<'p> Flow<'p> {
    /** `place = value`, or `place op= value` when `compound`, the assignment at `at`. */
    pub(super) fn assign(&mut self, place: &TPlace, compound: bool, value: &TExpr, at: Span) {
        let index = self.index_taint(place);
        let v = self.expr(value);
        let v = match compound {
            true => Shape::Leaf(v.all().join(self.read(place).all())),
            false => v,
        };
        self.write(place, v.raised(self.pc), index, at);
    }

    /** The labels of the indices along a place. */
    pub(super) fn index_taint(&mut self, place: &TPlace) -> Taint {
        let mut t = Taint::PUBLIC;
        for p in &place.proj {
            if let Proj::Index(i) = p {
                t = t.join(self.expr(i).all());
            }
        }
        t
    }

    /** The labels of the part a place names. */
    pub(super) fn read(&self, place: &TPlace) -> Shape {
        let root = self.env.get(place.root.0 as usize).cloned();
        let mut s = root.unwrap_or(Shape::Leaf(Taint::PUBLIC));
        for p in &place.proj {
            s = match p {
                Proj::TupleField(i) => s.child(*i as usize),
                Proj::Index(_) => s.elem(),
            };
        }
        s
    }

    /**
     * Set the part `place` names to `v`, every element on the way raised by `index`, and
     * meet the labels of the variable, the write being at `at`.
     */
    pub(super) fn write(&mut self, place: &TPlace, v: Shape, index: Taint, at: Span) {
        let root = place.root.0 as usize;
        let f = self.f;
        let Some(old) = self.env.get(root).cloned() else {
            return;
        };
        let ty = f.locals.get(root).map_or(Types::ERROR, |l| l.ty);
        let new = self.put(old, ty, &place.proj, v, index);
        let new = match f.locals.get(root) {
            Some(l) => self.labelled(new, &l.labels.0, ty, at),
            None => new,
        };
        if let Some(slot) = self.env.get_mut(root) {
            *slot = new;
        }
    }
}
