// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the STARK verification primitives. Includes the
//! real src/crypto source and checks it against its specification.

extern crate alloc;

pub mod crypto;

#[cfg(test)]
mod air_tests;
#[cfg(test)]
mod compose_step_tests;
// field_ext_tests + field_tests disabled: files lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod family_tests;
#[cfg(test)]
mod fold_witness_tests;
#[cfg(test)]
mod fri_ext_tests;
#[cfg(test)]
mod fri_poseidon_ext_tests;
#[cfg(test)]
mod golden_vk_tests;
mod witness_satisfies;

/// The satisfaction walk, for binaries that gate a shape before shipping it.
pub fn witness_satisfies_public(air: &(impl crypto::stark::air::Air + Sync), witness: &[crypto::stark::field::Fp]) -> bool {
    witness_satisfies::satisfies(air, witness)
}

/// The deployed join-split engine alone, for binaries that derive its
/// registration constants.
pub fn shield_deployed_wired() -> crypto::stark::air::WiredMultiGen {
    shield::test::scenario::balanced_deployed(shield::key::Break::None).wired
}
// fri_poseidon_tests disabled: file lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod fri_tests;
#[cfg(test)]
mod merkle_tests;
// ntt_tests disabled: file lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod parallel_bytes_tests;
#[cfg(test)]
mod periodic_z_tests;
#[cfg(test)]
mod poseidon_pre_tests;
// poly_tests disabled: file lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod poseidon_constants_gen;
// poseidon_merkle_tests disabled: file lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod dims_groups_tests;
#[cfg(test)]
mod dims_kind_tests;
#[cfg(test)]
mod dims_recursion_tests;
#[cfg(test)]
mod dims_tests;
#[cfg(test)]
mod dims_transfer_tests;
#[cfg(test)]
mod preprocessed_tests;
#[cfg(test)]
mod production_vector_gen;
pub mod recursion_assembly;
#[cfg(test)]
mod recursion_assembly_tests;
#[cfg(test)]
mod seam2_tests;
pub mod shield;
pub mod shield_params;
#[cfg(test)]
mod spec_out;
#[cfg(test)]
mod stark_selftest_gen;
#[cfg(test)]
mod step_assembly_tests;
#[cfg(test)]
mod vk_stability_tests;

// Machine-checked proof harnesses, compiled only under `cargo kani`.
#[cfg(kani)]
mod kani_proofs;
