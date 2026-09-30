/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Evaluating an expression: the dispatch over its kinds. */

use super::machine::{fail, Eval, Flow, Interp};
use super::{FailKind, Value};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'e> Interp<'e> {
    /** Evaluate `e`. */
    pub(super) fn eval(&mut self, e: &TExpr) -> Eval {
        self.enter(e.span)?;
        let out = self.eval_kind(e);
        self.leave();
        out
    }

    fn eval_kind(&mut self, e: &TExpr) -> Eval {
        let at = e.span;
        match &e.kind {
            TExprKind::Lit(l) => Ok(self.lit(*l, e.ty)),
            TExprKind::Local(l) => self
                .frame
                .get(l.0 as usize)
                .cloned()
                .ok_or_else(|| fail(FailKind::Internal, at)),
            TExprKind::Const(c) => self
                .env
                .konst(*c)
                .cloned()
                .ok_or_else(|| fail(FailKind::Internal, at)),
            TExprKind::Unary(op, a) => {
                let v = self.eval(a)?;
                self.unary(*op, v, a).map_err(|k| fail(k, at))
            }
            TExprKind::Chain(first, links) => self.chain(first, links),
            TExprKind::Cast(a) => {
                let v = self.eval(a)?;
                Ok(self.cast(v, e.ty))
            }
            TExprKind::Call(f, args) => self.eval_call(*f, args, at),
            TExprKind::Builtin(b, args) => self.builtin(*b, args, e),
            TExprKind::Tuple(es) => Ok(Value::Tuple(self.eval_all(es)?)),
            TExprKind::Array(es) => Ok(Value::Array(self.eval_all(es)?)),
            TExprKind::Repeat(a, n) => self.eval_repeat(a, *n, at),
            TExprKind::TupleField(a, i) => self.eval_field(a, *i, at),
            TExprKind::Index(a, i) => self.eval_index(a, i, at),
            TExprKind::Block(b) => self.eval_block(b),
            TExprKind::If(branches, last) => self.eval_if(branches, last.as_ref()),
            TExprKind::ForRange { .. } | TExprKind::ForArray { .. } | TExprKind::While { .. } => {
                self.eval_loop(e)
            }
            TExprKind::Break => Err(Flow::Break),
            TExprKind::Continue => Err(Flow::Continue),
            TExprKind::Return(v) => {
                let v = v.as_ref().map_or(Ok(Value::Unit), |v| self.eval(v))?;
                Err(Flow::Return(v))
            }
            TExprKind::Assign { place, op, value } => self.assign(place, *op, value),
            TExprKind::Declassify(a) => self.eval(a),
            TExprKind::Error => Err(fail(FailKind::Internal, at)),
        }
    }
}
