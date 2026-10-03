/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Messages for the ways a proving run fails. */

use nonos_zkolang::{render_error, ProveError, RunError};

/**
 * A one-line human message for a run failure. An unprovable statement is the honest
 * result for a false claim, so it says so plainly rather than printing a debug dump.
 */
pub(crate) fn render_run(src: &str, e: &RunError) -> String {
    match e {
        RunError::Compile(c) => render_error(src, c),
        RunError::Execute(ProveError::Unprovable { step }) => {
            format!("unprovable: no witness satisfies the statement (step {step})")
        }
        RunError::Execute(pe) => format!("cannot run: {pe:?}"),
        RunError::Layout(be) => format!("cannot lay out the trace: {be:?}"),
        RunError::ProgramTooLong { steps } => format!("program too long: {steps} steps"),
        RunError::InputCount {
            public_expected,
            public_got,
            secret_expected,
            secret_got,
        } => format!(
            "the program takes {public_expected} public inputs and {secret_expected} secrets, \
             but {public_got} and {secret_got} were given"
        ),
        RunError::TraceTooSmallToHide { log_trace_len } => {
            format!("trace too small to hide: log_trace_len {log_trace_len}")
        }
        RunError::InputNotInField { position } => {
            format!("input {position} is not below the field modulus")
        }
    }
}
