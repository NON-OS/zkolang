/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The labels of a value that may be either of two, part by part. */

use alloc::boxed::Box;

use super::shape::Shape;

impl Shape {
    /** The labels of a value that is one of two: each part the join of both. */
    pub fn join(&self, o: &Shape) -> Shape {
        match (self, o) {
            (Shape::Tuple(a), Shape::Tuple(b)) if a.len() == b.len() => {
                Shape::Tuple(a.iter().zip(b).map(|(x, y)| x.join(y)).collect())
            }
            (Shape::Array(a), Shape::Array(b)) => Shape::Array(Box::new(a.join(b))),
            (Shape::Leaf(t), s) | (s, Shape::Leaf(t)) => s.raised(*t),
            _ => Shape::Leaf(self.all().join(o.all())),
        }
    }
}
