// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use crate::crypto::stark::air::{GpGroup, RATE};
use crate::shield::key::{absorbed_cm_row, nullifier_edges, spend_pk_row};
use crate::shield::note::POOL_LOG_ROUNDS;
use crate::shield::wire::equate;
use alloc::vec::Vec;

/// The key that retires a note is the key that note committed to, and the
/// commitment it absorbs is the one membership authenticated. Without both, a
/// nullifier is a number the prover chose.
pub(crate) fn key_groups(l: &Layout) -> Vec<GpGroup> {
    let mut g = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;
    for (i, &base) in l.key.iter().enumerate() {
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
