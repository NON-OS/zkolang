/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The interpreter's state: the environment, the current function's locals, and the step
 * budget and recursion depth that bound every run.
 */

use alloc::vec::Vec;

use super::{Env, FailKind, Failure, Value};
use crate::compiler::source::Span;

/** How control leaves an expression other than with a value. */
pub(super) enum Flow {
    Break,
    Continue,
    Return(Value),
    Fail(Failure),
}

/** The result of evaluating an expression. */
pub(super) type Eval = Result<Value, Flow>;

/** An interpreter over one program. */
pub struct Interp<'e> {
    pub(super) env: &'e dyn Env,
    pub(super) frame: Vec<Value>,
    pub(super) steps: u64,
    pub(super) budget: u64,
    pub(super) depth: u32,
}

impl<'e> Interp<'e> {
    /** An interpreter that stops a run after `budget` steps. */
    pub fn new(env: &'e dyn Env, budget: u64) -> Interp<'e> {
        Interp {
            env,
            frame: Vec::new(),
            steps: 0,
            budget,
            depth: 0,
        }
    }
}

/** The flow of a failure of `kind` at `at`. */
pub(super) fn fail(kind: FailKind, span: Span) -> Flow {
    Flow::Fail(Failure { kind, span })
}
