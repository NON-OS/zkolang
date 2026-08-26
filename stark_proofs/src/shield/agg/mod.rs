// NONOS Operating System (AGPL-3.0-or-later)
//! The aggregation tree's carried state, per docs/16.
//!
//! Transfer proofs fan out; the state transition composes in order. A node
//! verifies its children and attests the move they make together, so the root is
//! one proof over one pair of states, which is what the pool swaps.

mod bind;
mod chain;
mod effect;
mod node;
mod read;
mod state;
#[cfg(test)]
mod test;
mod wire;

pub use bind::{cells, effect_classes, LANES};
pub use chain::{chain, leaf};
pub use effect::{induced, lift, Effect};
pub use node::{combine, Verified};
pub use read::{absorbed_at, read_effect};
pub use state::{Carried, Node};
pub use wire::realised;
