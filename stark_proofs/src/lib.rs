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
mod witness_satisfies;
#[cfg(test)]
mod golden_vk_tests;
#[cfg(test)]
mod fri_ext_tests;
#[cfg(test)]
mod fri_poseidon_ext_tests;
// fri_poseidon_tests disabled: file lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod fri_tests;
#[cfg(test)]
mod merkle_tests;
// ntt_tests disabled: file lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod periodic_z_tests;
// poly_tests disabled: file lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod poseidon_constants_gen;
// poseidon_merkle_tests disabled: file lost to a /tmp wipe (LOCAL only).
#[cfg(test)]
mod preprocessed_tests;
#[cfg(test)]
mod production_vector_gen;
#[cfg(test)]
mod recursion_assembly;
#[cfg(test)]
mod recursion_assembly_tests;
#[cfg(test)]
mod shield;
#[cfg(test)]
mod seam2_tests;
#[cfg(test)]
mod stark_selftest_gen;
#[cfg(test)]
mod step_assembly_tests;
#[cfg(test)]
mod dims_tests;
#[cfg(test)]
mod dims_recursion_tests;
#[cfg(test)]
mod dims_kind_tests;
#[cfg(test)]
mod dims_transfer_tests;
#[cfg(test)]
mod parallel_bytes_tests;
#[cfg(test)]
mod vk_stability_tests;

// Machine-checked proof harnesses, compiled only under `cargo kani`.
#[cfg(kani)]
mod kani_proofs;
