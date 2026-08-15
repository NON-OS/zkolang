// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::hasher;
use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::member::{PoolTree, TREE_DEPTH};

/// The contract precomputes this chain at deploy. If it drifts the circuit proves
/// membership in a different tree than the chain keeps.
#[test]
fn an_empty_pool_root_is_the_zeros_chain() {
    let h = hasher();
    let t = PoolTree::new(h.clone());
    let mut z = [Fp::ZERO; RATE];
    for _ in 0..TREE_DEPTH {
        z = h.compress(&z, &z);
    }
    assert_eq!(t.root(), z);
}
