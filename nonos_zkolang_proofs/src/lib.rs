/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Host-runnable proofs for the zkolang VM. A real VM run is laid out for the
//! ALU step AIR and driven through the money-grade Poseidon-committed STARK, so
//! the honest trace is proven to accept and each tampered trace to be rejected.

pub mod mimc;

#[cfg(test)]
mod array_tests;
#[cfg(test)]
mod backend_tests;
#[cfg(test)]
mod commit_tests;
#[cfg(test)]
mod const_table_tests;
#[cfg(test)]
mod corpus_tests;
#[cfg(test)]
mod feature_tests;
#[cfg(test)]
mod fn_tests;
#[cfg(test)]
mod hash_tests;
#[cfg(test)]
mod include_tests;
#[cfg(test)]
mod io_tests;
#[cfg(test)]
mod lang_tests;
#[cfg(test)]
mod library_tests;
#[cfg(test)]
mod logic_tests;
#[cfg(test)]
mod loop_tests;
#[cfg(test)]
mod nox_tests;
#[cfg(test)]
mod operator_tests;
#[cfg(test)]
mod recipes_tests;
#[cfg(test)]
mod register_file_tests;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
mod vkey_tests;
#[cfg(test)]
mod witness_tests;
