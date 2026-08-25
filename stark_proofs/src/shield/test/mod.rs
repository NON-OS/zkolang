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

mod depth;
mod deployed_depth;
mod fixture;
mod intents;
mod inventory;
mod roundtrip;
mod satisfies;
pub(crate) mod scenario;

mod commitment;
mod commitment_binding;
mod key_vector;
mod membership;
mod membership_scope;
mod pool_hash;
mod tree_zeros;

mod batch_assembly;
mod batch_price;
mod burns;
mod conserves;
mod foreign_key;
mod mints;
mod not_owner;
mod note_edge;
mod owns;
mod publics;
mod publics_scope;
mod unlisted;
