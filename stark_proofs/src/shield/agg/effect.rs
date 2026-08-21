// NONOS Operating System (AGPL-3.0-or-later)

use super::state::{Carried, Node};
use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;

/// What a verified transfer published: the two notes it retires and the two it
/// creates. These are public words of the proof, already bound to the cells that
/// computed them.
#[derive(Clone, Copy)]
pub(crate) struct Effect {
    pub nullifiers: [[Fp; RATE]; 2],
    pub outputs: [[Fp; RATE]; 2],
}

/// The transition a transfer makes, computed from what its proof published.
///
/// The node does not get to say where the chain went. A lift that verifies a
/// proof retiring Y and exposes a transition inserting X composes a fiction the
/// rest of the tree then carries as fact, and every proof in it still verifies.
/// So the exposed state is derived here and compared, never witnessed.
///
/// The roots move by absorbing; the real move is the IMT insert and the tree
/// append, and those are argued in shield::imt. What this pins is that the
/// exposed transition is a function of the proof's own effects.
pub(crate) fn induced(h: &Poseidon, old: Carried, e: &Effect) -> Carried {
    let mut nulls = old.nullifier_root;
    for n in &e.nullifiers {
        nulls = h.compress(&nulls, n);
    }
    let mut notes = old.note_root;
    for o in &e.outputs {
        notes = h.compress(&notes, o);
    }
    Carried { note_root: notes, next_index: old.next_index + 2, nullifier_root: nulls }
}

/// A leaf of the tree: verify a transfer, then expose the transition its own
/// effects make. `claimed` is what the node would publish; it has to be the one
/// the proof induces.
pub(crate) fn lift(h: &Poseidon, old: Carried, e: &Effect, claimed: Carried) -> Option<Node> {
    (induced(h, old, e) == claimed).then_some(Node { old, new: claimed })
}
