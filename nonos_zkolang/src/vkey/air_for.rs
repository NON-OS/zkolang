/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Build the wiring-only AIR for a program.

use super::KeyError;
use crate::air::{BuildError, StepAir};
use crate::driver::choose_log_t;
use crate::isa::Op;

/// The trace length before padding: the first halt's position plus one.
pub(super) fn step_count(program: &[Op]) -> Option<usize> {
    program
        .iter()
        .position(|op| matches!(op, Op::Halt))
        .map(|i| i + 1)
}

/// The canonical padded trace-length exponent for `program`, the same sizing the
/// verifier key and a real proof use. A recursive verifier must build its inner at
/// this `log_t` so the inner it attests matches `verifier_key(program, _)`; this is
/// the single source of that rule, so the two never drift.
pub fn program_log_t(program: &[Op]) -> Option<u32> {
    choose_log_t(step_count(program)?)
}

/// The wiring-only AIR, sized exactly as the driver sizes it, so the periodic columns
/// match a real proof's.
pub(super) fn air_for(program: &[Op]) -> Result<StepAir, KeyError> {
    let steps = step_count(program).ok_or(KeyError::NoHalt)?;
    let log_t = choose_log_t(steps).ok_or(KeyError::ProgramTooLong)?;
    StepAir::for_key(program, log_t).map_err(|e| match e {
        BuildError::NoHalt => KeyError::NoHalt,
        _ => KeyError::ProgramTooLong,
    })
}
