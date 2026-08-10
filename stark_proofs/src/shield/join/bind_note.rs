// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use crate::crypto::stark::air::{GpGroup, RATE};
use crate::shield::note::{note_edges, POOL_LOG_ROUNDS};
use crate::shield::wire::equate;
use alloc::vec::Vec;

/// Each note stays a chained compress tree, its committed value limbs are the
/// balance row's limbs, and each spent note is the leaf both trees walk from.
pub(crate) fn note_groups(l: &Layout) -> Vec<GpGroup> {
    let mut g = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;
    for (i, &base) in l.note.iter().enumerate() {
        for sw in note_edges(base, l.span_op) {
            let cols = if sw.1 == sw.3 { alloc::vec![sw.1] } else { alloc::vec![sw.1, sw.3] };
            g.push(equate(l.span, cols, &[sw]));
        }
        let bal = l.balance + i;
        g.push(equate(l.span, alloc::vec![0, 1], &[(bal, 1, base, 0)]));
        g.push(equate(l.span, alloc::vec![1, 2], &[(bal, 2, base, 1)]));
    }
    for (i, &m) in l.member.iter().enumerate() {
        let cm = l.note[i] + 2 * l.span_op + rounds;
        let lc = l.leaf_col[i];
        for c in 0..RATE {
            g.push(equate(l.span, alloc::vec![c, lc + c], &[(cm, c, m, lc + c)]));
        }
    }
    for (i, &a) in l.assoc.iter().enumerate() {
        let cm = l.note[i] + 2 * l.span_op + rounds;
        let lc = l.assoc_col[i];
        for c in 0..RATE {
            g.push(equate(l.span, alloc::vec![c, lc + c], &[(cm, c, a, lc + c)]));
        }
    }
    g
}
