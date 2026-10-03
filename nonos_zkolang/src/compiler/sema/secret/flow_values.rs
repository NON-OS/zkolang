/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The labels of the plainest values: a public value of a type, and a local's now. */

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::tir::TExpr;

impl<'p> Flow<'p> {
    /** A public value of `e`'s type. */
    pub(super) fn public(&self, e: &TExpr) -> Shape {
        Shape::of(e.ty, Taint::PUBLIC, &self.program.types)
    }

    /** The labels local `l` has now. */
    pub(super) fn local(&self, l: usize) -> Shape {
        self.env
            .get(l)
            .cloned()
            .unwrap_or(Shape::Leaf(Taint::PUBLIC))
    }
}
