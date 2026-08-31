// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{hasher, plain};
use super::satisfies::satisfies;
use crate::shield::member::{note_member, PoolTree};
use crate::shield::note::note_parts;

#[test]
fn a_deposited_note_reaches_the_pool_root() {
    let h = hasher();
    let mut t = PoolTree::new(h.clone());
    for s in 0..5u64 {
        t.insert(note_parts(&plain(s, 100 + s)).cm);
    }
    let cm = note_parts(&plain(3, 103)).cm;
    let (sibs, dirs) = t.path(3);
    let m = note_member(&h, cm, sibs, dirs, t.root());
    assert_eq!(m.proven_root, t.root());
    fn _typechecks(a: &crate::crypto::stark::air::WiredMultiGen, w: &[crate::crypto::stark::field::Fp]) -> bool { satisfies(a, w) }
}
