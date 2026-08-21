// NONOS Operating System (AGPL-3.0-or-later)
//! The aggregation tree's carried state, per docs/16.
//!
//! Transfer proofs fan out; the state transition composes in order. A node
//! verifies its children and attests the move they make together, so the root is
//! one proof over one pair of states, which is what the pool swaps.

mod chain;
mod effect;
mod node;
mod state;
#[cfg(test)]
mod test;

pub(crate) use chain::{chain, leaf};
pub(crate) use effect::{induced, lift, Effect};
pub(crate) use node::{combine, Verified};
pub(crate) use state::{Carried, Node};
