/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The prover's soundness parameters and the trace-size cap.

/// Queries, grinding bits, and extra blowup bits, matching the framework's own
/// money-grade tests: 32 queries, 16 grinding bits, 3 extra blowup bits.
pub(super) const QUERIES: usize = 32;
pub(super) const GRIND: u32 = 16;
pub(super) const BLOWUP: u32 = 3;

/// The largest trace this driver will size to, 2^16 rows. A program needing more
/// steps is rejected rather than silently proving a truncation.
pub(super) const MAX_LOG_T: u32 = 16;
