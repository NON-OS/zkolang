/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The labels of expressions (section 13.1): a literal or constant is public, and any
 * other value takes the labels of what it is computed from. `declassify` makes public.
 */

use alloc::boxed::Box;

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'p> Flow<'p> {
    /** The labels of the value of `e`. */
    pub(super) fn expr(&mut self, e: &TExpr) -> Shape {
        match &e.kind {
            TExprKind::Lit(_) | TExprKind::Const(_) | TExprKind::Error => self.public(e),
            TExprKind::Local(l) => self.local(l.0 as usize),
            TExprKind::Unary(_, a) | TExprKind::Cast(a) => Shape::Leaf(self.expr(a).all()),
            TExprKind::Chain(first, links) => self.chain(first, links),
            TExprKind::Builtin(_, args) => {
                let t = args
                    .iter()
                    .fold(Taint::PUBLIC, |t, a| t.join(self.expr(a).all()));
                Shape::of(e.ty, t, &self.program.types)
            }
            TExprKind::Tuple(es) => Shape::Tuple(es.iter().map(|x| self.expr(x)).collect()),
            TExprKind::Record(fs) => self.record(fs, e.ty),
            TExprKind::Array(es) => match es.iter().map(|x| self.expr(x)).reduce(|a, b| a.join(&b))
            {
                Some(el) => Shape::Array(Box::new(el)),
                None => self.public(e),
            },
            TExprKind::Repeat(a, _) => Shape::Array(Box::new(self.expr(a))),
            TExprKind::TupleField(a, i) => self.expr(a).child(*i as usize),
            TExprKind::Index(a, i) => {
                let a = self.expr(a);
                a.elem().raised(self.expr(i).all())
            }
            TExprKind::Block(b) => self.block(b),
            TExprKind::If(branches, last) => self.if_(branches, last.as_ref(), e.ty),
            TExprKind::ForRange { .. } | TExprKind::ForArray { .. } | TExprKind::While { .. } => {
                self.loop_(e);
                self.public(e)
            }
            TExprKind::Break | TExprKind::Continue => self.leave_loop(),
            TExprKind::Return(v) => self.return_(v.as_deref()),
            TExprKind::Assign { place, op, value } => {
                self.assign(place, op.is_some(), value, e.span);
                self.public(e)
            }
            TExprKind::Declassify(a) => {
                if self.expr(a).all().is_public() {
                    self.useless_declassify(e.span);
                }
                self.public(e)
            }
            TExprKind::Call(f, args) => self.call(*f, args, e),
        }
    }
}
