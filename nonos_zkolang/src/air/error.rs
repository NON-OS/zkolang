/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Why a program or VM trace could not be laid out for the step AIR.

/// The reasons `compile` or `build_trace` refuse a program or trace.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BuildError {
    /// The program has no reachable halt, so its length is undefined.
    NoHalt,
    /// The run is longer than the requested power-of-two trace length.
    TooLong { rows: usize, cap: usize },
    /// An `Out` names a public output index with no supplied value.
    MissingPublicOutput { idx: u16 },
}
