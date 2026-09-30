/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Whether a compiled test accepts, and what its outcome and the reference run make of it. */

use alloc::format;
use alloc::string::String;

use super::backend::backend;
use super::build_diag::{backend_failure, one};
use super::build_lower::lowering;
use super::witness::witness;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::interp::{FailKind, TestRun};
use crate::compiler::lower::lower_function;
use crate::compiler::source::Span;
use crate::compiler::tir::{FnId, TProgram};
use crate::Vm;

/** Whether the test `f` of `p`, compiled, runs to acceptance; or why it does not compile. */
pub(super) fn compiled_accepts(p: &TProgram, f: FnId) -> Result<bool, Diagnostics> {
    let at = p.fns.get(f.0 as usize).map_or(Span::DUMMY, |b| b.span);
    let ssa = lower_function(p, f).map_err(|e| lowering(e, at))?;
    let c = backend(&ssa).map_err(|e| backend_failure(e, at))?;
    let full = witness(&c, &[]).ok();
    Ok(full.is_some_and(|w| Vm::new().run(&c.machine.ops, &w, 0).is_ok()))
}

/** Why the test `r` of `p`, whose compiled run accepts if `accepted`, did not pass. */
pub(super) fn verdict(p: &TProgram, r: &TestRun, accepted: bool) -> Diagnostics {
    let at = p.fns.get(r.f.0 as usize).map_or(Span::DUMMY, |b| b.span);
    let d = match (&r.outcome, accepted) {
        (Err(f), _) if matches!(f.kind, FailKind::Budget | FailKind::Internal) => {
            let code = match f.kind {
                FailKind::Budget => Code::RUN_FAILED,
                _ => Code::INTERNAL,
            };
            let what = format!("the reference run stops: {}", f.kind.describe());
            Diagnostic::error(code, what, f.span, "stops here")
        }
        (Ok(_), false) | (Err(_), true) => {
            let what = String::from("the compiled test and the reference run disagree");
            Diagnostic::error(Code::INTERNAL, what, at, "this test")
        }
        (Ok(_), true) if r.should_fail => {
            let what = String::from("the test is marked `#[should_fail]`, and its run accepts");
            Diagnostic::error(Code::RUN_FAILED, what, at, "this test")
        }
        (Err(f), false) if !r.should_fail => {
            let what = format!("the run fails: {}", f.kind.describe());
            Diagnostic::error(Code::RUN_FAILED, what, f.span, "fails here")
        }
        _ => return Diagnostics::new(),
    };
    one(d)
}
