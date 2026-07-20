/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Prove a laid-out trace and verify it under the bound statement.

use nonos_stark::air::{
    stark_prove_poseidon_ext_pub, stark_verify_poseidon_ext_pub, Poseidon, RATE,
};
use nonos_stark::field::Fp;

use super::params::{BLOWUP, GRIND, QUERIES};
use crate::air::StepAir;

/// Prove the trace with the public statement seeded into the transcript, then verify
/// the proof against the same statement. The hasher is fixed, so prove and verify
/// replay one transcript.
pub(super) fn prove_verify(air: &StepAir, flat: &[Fp], publics: &[Fp]) -> bool {
    let hasher = Poseidon::new(2, [Fp::ZERO; RATE]);
    let proof = stark_prove_poseidon_ext_pub(air, flat, QUERIES, GRIND, BLOWUP, &hasher, publics);
    stark_verify_poseidon_ext_pub(air, &proof, QUERIES, GRIND, BLOWUP, &hasher, publics)
}
