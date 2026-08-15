// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{hasher, plain};
use crate::shield::member::{note_member, PoolTree};
use crate::shield::note::note_parts;

/// A tampered path is honest arithmetic that walks elsewhere, so the constraints
/// accept it. Membership rests on the walked root, which is why the assembly must
/// bind that root to the published one.
#[test]
fn a_tampered_path_walks_away_from_the_root() {
    let h = hasher();
    let mut t = PoolTree::new(h.clone());
    for s in 0..5u64 {
        t.insert(note_parts(&plain(s, 100 + s)).cm);
    }
    let cm = note_parts(&plain(3, 103)).cm;
    let (mut sibs, dirs) = t.path(3);
    sibs[0][0] = sibs[0][0] + crate::crypto::stark::field::Fp::ONE;
    let m = note_member(&h, cm, sibs, dirs, t.root());
    assert_ne!(m.proven_root, t.root());
}
