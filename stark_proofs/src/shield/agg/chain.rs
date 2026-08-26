// NONOS Operating System (AGPL-3.0-or-later)

use super::state::{Carried, Node};

/// Two children compose when the second started where the first ended.
///
/// The tree is a sequential composition, not an unordered pair. Without this both
/// children can start from the same state, the parent attests a move from that
/// state to the second child's end, and the first child's whole subtree leaves
/// the batch. Every proof verifies, the root is well formed, and those transfers
/// are gone.
///
/// One equality over the whole carried state, not a check per field: a field
/// added later is covered by having been added, rather than by someone
/// remembering to compare it.
pub fn chain(a: &Node, b: &Node) -> Option<Node> {
    if a.new != b.old {
        return None;
    }
    Some(Node {
        old: a.old,
        new: b.new,
    })
}

/// A leaf's claim, for the lift. One transfer moves the chain by its own two
/// nullifiers and two outputs.
pub fn leaf(old: Carried, new: Carried) -> Node {
    Node { old, new }
}
