/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Why a proving run did not complete. Each variant names the stage that failed
//! and carries that stage's own error, so a caller can tell a false claim (which
//! is `Execute`) from a malformed program (which is `Compile`).

use crate::air::BuildError;
use crate::lang::CompileError;
use crate::vm::ProveError;

/// The reasons a proving run stops short of a report.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RunError {
    /// The source did not compile.
    Compile(CompileError),
    /// The program ran but its witness violated a constraint (a failed assert, an
    /// inverse of zero), so there is no trace to prove. This is the honest result
    /// for a program whose claim is false.
    Execute(ProveError),
    /// The executed trace could not be laid out for the AIR.
    Layout(BuildError),
    /// The program needs more steps than the driver will size a trace to.
    ProgramTooLong { steps: usize },
}
