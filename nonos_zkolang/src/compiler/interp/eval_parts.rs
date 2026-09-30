/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Literals, and the operands a construct evaluates left to right. */

use alloc::vec;
use alloc::vec::Vec;

use super::machine::{fail, Eval, Flow, Interp};
use super::{FailKind, Value};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::tir::{TExpr, TLit};

impl<'e> Interp<'e> {
    /** The value of a literal of `e`'s type. */
    pub(super) fn lit(&self, l: TLit, ty: TyId) -> Value {
        match l {
            TLit::Unit => Value::Unit,
            TLit::Bool(b) => Value::Bool(b),
            TLit::Int(v) if *self.env.types().kind(ty) == TyKind::Field => Value::Field(v as u64),
            TLit::Int(v) => Value::Int(v),
        }
    }

    /** Evaluate `es` left to right. */
    pub(super) fn eval_all(&mut self, es: &[TExpr]) -> Result<Vec<Value>, Flow> {
        es.iter().map(|e| self.eval(e)).collect()
    }

    /** `[a; n]`, `a` evaluated once. */
    pub(super) fn eval_repeat(&mut self, a: &TExpr, n: u32, at: Span) -> Eval {
        let v = self.eval(a)?;
        self.spend(u64::from(n), at)?;
        Ok(Value::Array(vec![v; n as usize]))
    }

    /** `a.i` on a tuple. */
    pub(super) fn eval_field(&mut self, a: &TExpr, i: u32, at: Span) -> Eval {
        let v = self.eval(a)?;
        v.parts()
            .get(i as usize)
            .cloned()
            .ok_or_else(|| fail(FailKind::Internal, at))
    }

    /** `a[i]`, which fails when `i` is out of bounds. */
    pub(super) fn eval_index(&mut self, a: &TExpr, i: &TExpr, at: Span) -> Eval {
        let v = self.eval(a)?;
        let i = self.eval(i)?.int();
        let got = usize::try_from(i)
            .ok()
            .and_then(|i| v.parts().get(i).cloned());
        got.ok_or_else(|| fail(FailKind::IndexOutOfBounds, at))
    }
}
