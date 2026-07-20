/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Why a run produced no valid trace. Every variant is a legitimate outcome the
//! caller can inspect, never a panic.

/// The reasons the executor stops without a provable trace. `Unprovable` is not a
/// bug: it means the witness did not satisfy the program's constraints, which is
/// the honest result for a program whose claim is false.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProveError {
    /// A register index outside `0..REGS`.
    BadRegister(u8),
    /// An input index past the supplied input vector.
    BadInput(u16),
    /// The program ran its whole instruction list without a `Halt`.
    NoHalt,
    /// A constraint the trace must satisfy did not hold, at this step.
    Unprovable { step: u64 },
}
