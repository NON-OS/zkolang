// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{GpGroup, RATE};
use crate::shield::key::{absorbed_cm_row, nullifier_edges, spend_pk_row};
use crate::shield::note::{note_edges, POOL_LOG_ROUNDS};
use crate::shield::wire::equate;
use alloc::vec::Vec;

pub(crate) struct Layout {
    pub span: usize,
    pub span_op: usize,
    pub note: Vec<usize>,
    pub member: Vec<usize>,
    pub key: Vec<usize>,
    pub key_span: Vec<usize>,
    pub leaf_col: Vec<usize>,
    pub balance: usize,
}

/// Value is conserved over the values the notes committed to, the spent notes are
/// the notes membership proved, and the key that retires each note is the key that
/// note committed to. Each equality is explicit; none is implied by the regions.
pub(crate) fn groups(l: &Layout) -> Vec<GpGroup> {
    let mut g = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;

    for (i, &base) in l.note.iter().enumerate() {
        for sw in note_edges(base, l.span_op) {
            let cols =
                if sw.1 == sw.3 { alloc::vec![sw.1] } else { alloc::vec![sw.1, sw.3] };
            g.push(equate(l.span, cols, &[sw]));
        }
        let bal = l.balance + i;
        g.push(equate(l.span, alloc::vec![0, 1], &[(bal, 1, base, 0)]));
        g.push(equate(l.span, alloc::vec![1, 2], &[(bal, 2, base, 1)]));
    }

    for i in 0..l.member.len() {
        let cm = l.note[i] + 2 * l.span_op + rounds;
        let lc = l.leaf_col[i];
        for c in 0..RATE {
            g.push(equate(l.span, alloc::vec![c, lc + c], &[(cm, c, l.member[i], lc + c)]));
        }
    }

    for i in 0..l.key.len() {
        let base = l.key[i];
        for sw in nullifier_edges(base, l.key_span[i]) {
            g.push(equate(l.span, alloc::vec![sw.1], &[sw]));
        }
        let cm = l.note[i] + 2 * l.span_op + rounds;
        for c in 0..RATE {
            g.push(equate(
                l.span,
                alloc::vec![c, 3 + c],
                &[(spend_pk_row(base), c, l.note[i], 3 + c)],
            ));
            g.push(equate(
                l.span,
                alloc::vec![c, RATE + c],
                &[(cm, c, absorbed_cm_row(base, l.key_span[i]), RATE + c)],
            ));
        }
    }
    g
}
