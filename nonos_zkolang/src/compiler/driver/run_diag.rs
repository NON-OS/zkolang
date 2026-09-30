/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The diagnostic of a run that did not end in a result or a verified proof. */

use alloc::format;
use alloc::string::String;

use super::abi::AbiError;
use super::built::Built;
use super::run::RunFailure;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;

/** What went wrong in a run of `b`, as a diagnostic. */
pub fn diagnose(b: &Built, f: &RunFailure) -> Diagnostic {
    let main = b.program.main.and_then(|m| b.program.fns.get(m.0 as usize));
    let at = main.map_or(Span::DUMMY, |f| f.span);
    let internal =
        |what: String| Diagnostic::error(Code::INTERNAL, what, at, "running this program");
    match f {
        RunFailure::Inputs(e, secret) => {
            let side = if *secret { "secret" } else { "public" };
            let what = match e {
                AbiError::Count { expected, got } => {
                    let values = if *expected == 1 { "value" } else { "values" };
                    format!("`main` takes {expected} {side} {values}, and {got} were given")
                }
                AbiError::Range { position } => {
                    format!("{side} value {position} is not a value of its type")
                }
            };
            Diagnostic::error(Code::BAD_INPUTS, what, at, "these parameters")
        }
        RunFailure::Fails(f) => Diagnostic::error(
            Code::RUN_FAILED,
            format!("the run fails: {}", f.kind.describe()),
            f.span,
            "fails here",
        ),
        RunFailure::Prove(e) => internal(format!("the prover stopped: {e:?}")),
        RunFailure::Disagree => internal(String::from(
            "the compiled program and the reference run disagree",
        )),
        RunFailure::Unverified => internal(String::from("the proof did not verify")),
    }
}
