/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Pay-to-prove metering in NOX. A proof is a service, and this turns a proving run
//! into a deterministic price: a small anti-spam floor plus a rate on the trace area,
//! split so most of the fee pays the prover and a protocol cut accrues to the NOX
//! treasury. Trace area, rows times width, is the honest cost driver, so charging on
//! it aligns price with work and cannot be gamed by padding. Every rate is a
//! governance-tunable constant, not a fixed law.

mod price;
mod quote_type;

pub use price::quote;
pub use quote_type::Quote;

/// microNOX per whole NOX, so small proofs still carry a non-zero price.
pub const MICRONOX_PER_NOX: u64 = 1_000_000;

/// A flat floor on every proof, so submitting work is never free. 0.001 NOX.
pub(crate) const BASE_MICRONOX: u64 = 1_000;

/// The rate on proving work: microNOX per thousand trace cells. A cell is one field
/// element of the committed trace.
pub(crate) const MICRONOX_PER_KCELL: u64 = 50;

/// The protocol's cut of each fee, in basis points, accruing to the treasury. 5%.
pub(crate) const PROTOCOL_FEE_BPS: u64 = 500;
