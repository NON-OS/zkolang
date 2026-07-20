/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The rate-pinned market keys.

use super::periodic_root::periodic_root;
use super::verifier_key::verifier_key;
use super::{KeyError, REGISTRATION_RATE};
use crate::isa::Op;

/// The verifier key at the market's fixed registration rate, the value a NOX proving
/// market stores against a program commitment.
pub fn registration_key(program: &[Op]) -> Result<[u8; 32], KeyError> {
    verifier_key(program, REGISTRATION_RATE)
}

/// The preprocessed-periodic root at the registration rate, the tree a conforming
/// proof commits its periodic columns with.
pub fn registration_root(program: &[Op]) -> Result<[u8; 32], KeyError> {
    periodic_root(program, REGISTRATION_RATE)
}
