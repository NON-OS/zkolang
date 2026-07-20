/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The preprocessed-periodic root of a program's step AIR.

use nonos_stark::air::periodic_root as air_periodic_root;

use super::air_for::air_for;
use super::KeyError;
use crate::isa::Op;

/// The exact tree the preprocessed prover commits, so a registered root and a proof's
/// committed root cannot diverge. `extra_blowup_bits` must match the recursion's
/// inner-proof rate.
pub fn periodic_root(program: &[Op], extra_blowup_bits: u32) -> Result<[u8; 32], KeyError> {
    Ok(air_periodic_root(&air_for(program)?, extra_blowup_bits))
}
