/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What a call needs to know of a function's secret flow, written in the slots of its
 * parameters: the labels of its result, part by part; the slots whose arguments must be
 * public; and the labels each `&mut` parameter's final value has.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::shape::Shape;
use super::taint::Taint;

/** A function's secret flow, in terms of its parameters' slots. */
#[derive(Clone, Debug)]
pub struct Summary {
    pub result: Shape,
    /** The slots whose arguments must be public, as bits. */
    pub needs_public: u128,
    /** For each parameter, the labels of its final value if it is a `&mut` parameter. */
    pub ref_out: Vec<Option<Shape>>,
}

impl Shape {
    /** This shape with each slot's label replaced by `args(k)`. */
    pub fn apply(&self, args: &dyn Fn(usize) -> Taint) -> Shape {
        match self {
            Shape::Leaf(t) => Shape::Leaf(t.apply(args)),
            Shape::Tuple(ps) => Shape::Tuple(ps.iter().map(|p| p.apply(args)).collect()),
            Shape::Array(e) => Shape::Array(Box::new(e.apply(args))),
        }
    }
}
