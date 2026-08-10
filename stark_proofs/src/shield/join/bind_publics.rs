// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use super::publics::{
    ASSET_ID, ASSOC_ROOT, FEE, NF0, NF1, NOTE_ROOT, OUT_CM0, OUT_CM1, PUBLIC_AMOUNT,
};
use crate::crypto::stark::air::{GpGroup, RATE};
use crate::shield::member::TREE_DEPTH;
use crate::shield::note::POOL_LOG_ROUNDS;
use crate::shield::wire::equate;
use alloc::vec::Vec;

/// Every word is tied to the cell that computes it. A tamper test alone would
/// only show a word is constrained to something, not to the right something.
pub(crate) fn public_groups(l: &Layout, pub_off: usize) -> Vec<GpGroup> {
    let mut g = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;
    let word = |i: usize| pub_off + i;

    // The root the pool published is the root membership walked to.
    let walked = l.member[0] + TREE_DEPTH * rounds;
    for c in 0..RATE {
        g.push(equate(l.span, alloc::vec![0, c], &[(word(NOTE_ROOT + c), 0, walked, c)]));
    }

    // Each retired nullifier is the one its key hierarchy produced.
    for (i, &base) in l.key.iter().enumerate() {
        let nf = base + 3 * l.key_span[i] + rounds;
        let at = if i == 0 { NF0 } else { NF1 };
        for c in 0..RATE {
            g.push(equate(l.span, alloc::vec![0, c], &[(word(at + c), 0, nf, c)]));
        }
    }

    // Each created commitment is the one the output note committed to.
    for (i, at) in [OUT_CM0, OUT_CM1].into_iter().enumerate() {
        let cm = l.note[2 + i] + 2 * l.span_op + rounds;
        for c in 0..RATE {
            g.push(equate(l.span, alloc::vec![0, c], &[(word(at + c), 0, cm, c)]));
        }
    }

    // The public legs are the recomposed values the balance summed, not numbers
    // restated beside it.
    g.push(equate(l.span, alloc::vec![0, 3], &[(word(PUBLIC_AMOUNT), 0, l.balance + 4, 3)]));
    g.push(equate(l.span, alloc::vec![0, 3], &[(word(FEE), 0, l.balance + 5, 3)]));
    g.push(equate(l.span, alloc::vec![0, 2], &[(word(ASSET_ID), 0, l.note[0], 2)]));

    let walked_assoc = l.assoc[0] + TREE_DEPTH * rounds;
    for c in 0..RATE {
        g.push(equate(l.span, alloc::vec![0, c], &[(word(ASSOC_ROOT + c), 0, walked_assoc, c)]));
    }
    g
}
