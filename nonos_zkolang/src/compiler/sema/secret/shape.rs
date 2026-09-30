/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The labels of a value's parts: a tuple's fields each have their own, and an array's
 * elements share one, since a run-time index may name any of them.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::taint::Taint;
use crate::compiler::sema::ty::{TyId, TyKind, Types};

/** The labels of a value, part by part; a leaf stands for every part below it. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Shape {
    Leaf(Taint),
    Tuple(Vec<Shape>),
    Array(Box<Shape>),
}

impl Shape {
    /** A value of type `ty` whose every part has label `t`. */
    pub fn of(ty: TyId, t: Taint, types: &Types) -> Shape {
        if let Some(ts) = types.record(ty) {
            return Shape::Tuple(ts.iter().map(|&e| Shape::of(e, t, types)).collect());
        }
        match types.kind(ty) {
            TyKind::Array(e, _) => Shape::Array(Box::new(Shape::of(*e, t, types))),
            _ => Shape::Leaf(t),
        }
    }

    /** The label of the value as a whole: the join of its parts'. */
    pub fn all(&self) -> Taint {
        match self {
            Shape::Leaf(t) => *t,
            Shape::Tuple(ps) => ps.iter().fold(Taint::PUBLIC, |a, p| a.join(p.all())),
            Shape::Array(e) => e.all(),
        }
    }

    /** Every part raised by `t`. */
    pub fn raised(&self, t: Taint) -> Shape {
        match self {
            Shape::Leaf(x) => Shape::Leaf(x.join(t)),
            Shape::Tuple(ps) => Shape::Tuple(ps.iter().map(|p| p.raised(t)).collect()),
            Shape::Array(e) => Shape::Array(Box::new(e.raised(t))),
        }
    }

    /** The labels of field `i` of a tuple. */
    pub fn child(&self, i: usize) -> Shape {
        match self {
            Shape::Tuple(ps) => ps.get(i).cloned().unwrap_or(Shape::Leaf(self.all())),
            _ => Shape::Leaf(self.all()),
        }
    }

    /** The labels an array's elements share. */
    pub fn elem(&self) -> Shape {
        match self {
            Shape::Array(e) => (**e).clone(),
            _ => Shape::Leaf(self.all()),
        }
    }
}
