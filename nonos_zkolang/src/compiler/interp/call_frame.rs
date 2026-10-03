/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A call's own frame: the callee's locals, its parameters bound, and its body run. */

use alloc::vec;
use alloc::vec::Vec;

use super::machine::{fail, Flow, Interp};
use super::{FailKind, Value};
use crate::compiler::source::Span;
use crate::compiler::tir::FnId;

impl<'e> Interp<'e> {
    /** Run `f` on `args`; its result, and the final value of each parameter. */
    pub(super) fn call_values(
        &mut self,
        f: FnId,
        args: Vec<Value>,
        at: Span,
    ) -> Result<(Value, Vec<Value>), Flow> {
        let env = self.env;
        let body = env.body(f).ok_or_else(|| fail(FailKind::Internal, at))?;
        let caller = core::mem::replace(&mut self.frame, vec![Value::Unit; body.locals.len()]);
        for (p, v) in body.params.iter().zip(args) {
            self.set(p.local.0, v.clone());
            self.bind(&p.pat, v);
        }
        let out = match self.eval_block(&body.body) {
            Ok(v) | Err(Flow::Return(v)) => Ok(v),
            Err(Flow::Fail(e)) => Err(Flow::Fail(e)),
            Err(_) => Err(fail(FailKind::Internal, at)),
        };
        let finals = body
            .params
            .iter()
            .map(|p| {
                self.frame
                    .get(p.local.0 as usize)
                    .cloned()
                    .unwrap_or(Value::Unit)
            })
            .collect();
        self.frame = caller;
        out.map(|v| (v, finals))
    }
}
