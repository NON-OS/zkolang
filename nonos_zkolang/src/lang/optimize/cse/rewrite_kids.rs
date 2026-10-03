/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Rewriting a node's children, in the order its structural key lists them. */

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use super::rewrite::Rewrite;
use crate::lang::parse::Expr;

type Two = fn(Box<Expr>, Box<Expr>) -> Expr;
type Three = fn(Box<Expr>, Box<Expr>, Box<Expr>) -> Expr;

impl Rewrite<'_> {
    pub(super) fn one(&mut self, x: &Expr, f: fn(Box<Expr>) -> Expr) -> (Expr, Vec<usize>) {
        let (x, i) = self.go(x);
        (f(Box::new(x)), vec![i])
    }

    pub(super) fn two(&mut self, a: &Expr, b: &Expr, f: Two) -> (Expr, Vec<usize>) {
        let ((a, i), (b, j)) = (self.go(a), self.go(b));
        (f(Box::new(a), Box::new(b)), vec![i, j])
    }

    pub(super) fn three(&mut self, c: &Expr, a: &Expr, b: &Expr, f: Three) -> (Expr, Vec<usize>) {
        let ((c, h), (a, i), (b, j)) = (self.go(c), self.go(a), self.go(b));
        (f(Box::new(c), Box::new(a), Box::new(b)), vec![h, i, j])
    }

    pub(super) fn many(
        &mut self,
        xs: &[Expr],
        f: impl FnOnce(Vec<Expr>) -> Expr,
    ) -> (Expr, Vec<usize>) {
        let (xs, kids): (Vec<Expr>, Vec<usize>) = xs.iter().map(|x| self.go(x)).unzip();
        (f(xs), kids)
    }
}
