// NONOS Operating System (AGPL-3.0-or-later)

//! Two tiers. The fast tier runs on every change: the deployed hash, the
//! commitment, the tree convention, the published key vector, and the binding
//! inventory itself.
//!
//! The assembly forgeries are marked ignore and are the RELEASE gate, not the
//! PR gate. Each builds a whole join split at the deployed round count, which is
//! minutes apiece. They are the fourteen in inventory.rs, they must all reject
//! before an emit, and they are what the single permutation argument has to keep
//! rejecting:
//!
//!   cargo test --release -p stark_proofs shield::test -- --ignored

pub mod depth;
#[cfg(test)]
mod cross_asset;
#[cfg(test)]
mod double_spend;
#[cfg(test)]
mod deployed_depth;
pub mod fixture;
#[cfg(test)]
mod intents;
#[cfg(test)]
mod inventory;
#[cfg(test)]
mod roundtrip;
#[cfg(test)]
mod satisfies;
pub mod scenario;

#[cfg(test)]
mod commitment;
#[cfg(test)]
mod commitment_binding;
#[cfg(test)]
mod key_vector;
#[cfg(test)]
mod membership;
#[cfg(test)]
mod membership_scope;
#[cfg(test)]
mod pool_hash;
#[cfg(test)]
mod tree_zeros;

#[cfg(test)]
mod batch_assembly;
#[cfg(test)]
mod batch_price;
#[cfg(test)]
mod burns;
#[cfg(test)]
mod conserves;
#[cfg(test)]
mod foreign_key;
#[cfg(test)]
mod mints;
#[cfg(test)]
mod not_owner;
#[cfg(test)]
mod note_edge;
#[cfg(test)]
mod owns;
#[cfg(test)]
mod publics;
#[cfg(test)]
mod publics_scope;
#[cfg(test)]
mod unlisted;
