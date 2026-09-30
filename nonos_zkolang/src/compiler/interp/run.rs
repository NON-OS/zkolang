/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Entry points: running a function, and evaluating a constant. */

use alloc::vec;
use alloc::vec::Vec;

use super::machine::{Flow, Interp};
use super::{FailKind, Failure, Value};
use crate::compiler::source::Span;
use crate::compiler::tir::{FnId, TExpr};

impl<'e> Interp<'e> {
    /** Run function `f` on `args`, its parameters in order. */
    pub fn call(&mut self, f: FnId, args: Vec<Value>, at: Span) -> Result<Value, Failure> {
        let out = self.call_values(f, args, at).map(|(v, _)| v);
        out.map_err(|flow| self.failure(flow, at))
    }

    /** Evaluate `e`, in a frame of `locals` slots, as a constant. */
    pub fn eval_const(&mut self, e: &TExpr, locals: usize) -> Result<Value, Failure> {
        let saved = core::mem::replace(&mut self.frame, vec![Value::Unit; locals]);
        let out = self.eval(e);
        self.frame = saved;
        out.map_err(|flow| self.failure(flow, e.span))
    }

    /** The failure a flow that left the top level stands for. */
    fn failure(&self, flow: Flow, at: Span) -> Failure {
        match flow {
            Flow::Fail(f) => f,
            _ => Failure {
                kind: FailKind::Internal,
                span: at,
            },
        }
    }
}
