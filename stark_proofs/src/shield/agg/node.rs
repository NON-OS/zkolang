// NONOS Operating System (AGPL-3.0-or-later)

use super::chain::chain;
use super::state::Node;

/// A child as its parent sees it: the proof was verified, and this is the
/// transition that proof exposed.
///
/// The pairing is the point. A node that re-witnesses a child's transition can
/// verify one proof and compose a different move, so the state carried up is not
/// the state anything proved. Carrying the exposed value is what keeps the chain
/// about the proofs beneath it.
#[derive(Clone, Copy)]
pub struct Verified {
    pub exposed: Node,
}

/// Combine two verified children. The transitions compared are the ones the
/// children exposed, not values the node was handed.
pub fn combine(a: &Verified, b: &Verified) -> Option<Node> {
    chain(&a.exposed, &b.exposed)
}
