// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use crate::crypto::stark::air::RATE;
use crate::shield::note::{note_edges, POOL_LOG_ROUNDS};
use crate::shield::wire_class::{pair, Class};
use alloc::vec::Vec;

/// Each note stays a chained compress tree, its committed value limbs are the
/// balance row's limbs, and each spent note is the leaf both trees walk from.
pub(crate) fn note_classes(l: &Layout) -> Vec<Class> {
    let mut c = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;
    for (i, &base) in l.note.iter().enumerate() {
        for sw in note_edges(base, l.span_op) {
            c.push(pair(sw.0, sw.1, sw.2, sw.3));
        }
        let bal = l.balance + i;
        c.push(pair(bal, 1, base, 0));
        c.push(pair(bal, 2, base, 1));
    }
    for (i, &m) in l.member.iter().enumerate() {
        let cm = l.note[i] + 2 * l.span_op + rounds;
        let lc = l.leaf_col[i];
        for k in 0..RATE {
            c.push(pair(cm, k, m, lc + k));
        }
    }
    for (i, &a) in l.assoc.iter().enumerate() {
        let cm = l.note[i] + 2 * l.span_op + rounds;
        let lc = l.assoc_col[i];
        for k in 0..RATE {
            c.push(pair(cm, k, a, lc + k));
        }
    }
    c
}
