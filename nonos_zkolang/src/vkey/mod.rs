/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The per-program verifier key: what a proving market registers against a program
//! commitment so the on-chain statement binds the program's wiring, not just its
//! bytes. The step AIR's periodic columns are the program's data-flow wiring, and
//! the key ties the preprocessed periodic root to the commitment, a deterministic
//! public relation a challenger recomputes. One function per file.

mod air_for;
mod periodic_root;
mod registration;
mod verifier_key;

pub use periodic_root::periodic_root;
pub use registration::{registration_key, registration_root};
pub use verifier_key::verifier_key;

/// The wiring-derivation and descriptor-layout version. Bump it if the derivation or
/// the field order ever changes, so a change forces a new key.
pub(crate) const WIRING_VERSION: u8 = 1;

/// The FRI blowup every key is registered at. The recursion's inner proof and the
/// on-chain contract fix this immutably, so a key at any other rate never matches a
/// real proof. Use this rather than a rate literal; the rate is part of the key.
pub const REGISTRATION_RATE: u32 = 3;

/// Why a verifier key could not be derived.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyError {
    /// The program has no reachable halt, so its trace length is undefined.
    NoHalt,
    /// The program needs a trace longer than the driver will size to.
    ProgramTooLong,
}
