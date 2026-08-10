// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use crate::crypto::stark::air::RATE;
use crate::shield::key::{absorbed_cm_row, nullifier_edges, spend_pk_row};
use crate::shield::note::POOL_LOG_ROUNDS;
use crate::shield::wire_class::{pair, Class};
use alloc::vec::Vec;

/// The key that retires a note is the key that note committed to, and the
/// commitment it absorbs is the one membership authenticated. Without both, a
/// nullifier is a number the prover chose.
pub(crate) fn key_classes(l: &Layout) -> Vec<Class> {
    let mut c = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;
    for (i, &base) in l.key.iter().enumerate() {
        for sw in nullifier_edges(base, l.key_span[i]) {
            c.push(pair(sw.0, sw.1, sw.2, sw.3));
        }
        let cm = l.note[i] + 2 * l.span_op + rounds;
        for k in 0..RATE {
            c.push(pair(spend_pk_row(base), k, l.note[i], 3 + k));
            c.push(pair(cm, k, absorbed_cm_row(base, l.key_span[i]), RATE + k));
        }
    }
    c
}
