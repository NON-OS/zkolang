/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Lowering an expression to the slots of its value, the dispatch over its kinds. */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::ssa::V;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'p> Lower<'p> {
    /** The slots of the value of `e`, evaluated where the guard says the point runs. */
    pub(super) fn expr(&mut self, e: &TExpr) -> L<Vec<V>> {
        if self.b.over {
            return Err(LowerError::TooLarge);
        }
        Ok(match &e.kind {
            TExprKind::Lit(l) => self.lit(*l, e.ty),
            TExprKind::Local(l) => self.local(*l),
            TExprKind::Const(c) => {
                let p = self.p;
                let v = p.values.get(c.0 as usize).ok_or(LowerError::Unsupported(
                    "a constant that did not evaluate",
                    e.span,
                ))?;
                self.value_slots(v, e.ty)
            }
            TExprKind::Unary(op, a) => self.unary(*op, a, e)?,
            TExprKind::Chain(first, links) => self.chain(first, links)?,
            TExprKind::Cast(a) => self.cast(a, e)?,
            TExprKind::Call(f, args) => self.call(*f, args)?,
            TExprKind::Builtin(b, args) => self.builtin(*b, args, e)?,
            TExprKind::Tuple(_)
            | TExprKind::Array(_)
            | TExprKind::Repeat(..)
            | TExprKind::TupleField(..) => self.compound(e)?,
            TExprKind::Index(a, i) => self.index(a, i)?,
            TExprKind::Block(b) => self.block(b)?,
            TExprKind::If(branches, last) => self.if_(branches, last.as_ref(), e.ty)?,
            TExprKind::ForRange { .. } | TExprKind::ForArray { .. } | TExprKind::While { .. } => {
                self.loop_(e)?;
                Vec::new()
            }
            TExprKind::Break => self.exit_loop(true),
            TExprKind::Continue => self.exit_loop(false),
            TExprKind::Return(v) => self.return_(v.as_deref())?,
            TExprKind::Assign { place, op, value } => {
                self.assign_expr(place, *op, value)?;
                Vec::new()
            }
            TExprKind::Declassify(a) => self.expr(a)?,
            TExprKind::Error => {
                return Err(LowerError::Unsupported(
                    "an expression that failed to check",
                    e.span,
                ))
            }
        })
    }
}
