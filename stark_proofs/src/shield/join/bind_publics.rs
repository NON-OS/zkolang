// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use super::publics::{
    ASSET_ID, ASSOC_ROOT, FEE, NF0, NF1, NOTE_ROOT, OUT_CM0, OUT_CM1, PUBLIC_AMOUNT,
};
use crate::crypto::stark::air::Cell;
use crate::crypto::stark::air::RATE;
use crate::shield::note::POOL_LOG_ROUNDS;
use crate::shield::wire_class::{pair, Class};
use alloc::vec::Vec;

/// Every word is tied to the cell that computes it. A tamper test alone would
/// only show a word is constrained to something, not to the right something.
pub(crate) fn public_classes(l: &Layout, pub_off: usize) -> Vec<Class> {
    let mut g = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;
    let word = |i: usize| pub_off + i;

    // The published root is the root every membership walked to. Tie only the
    // first and the second walks to a root nobody named. One class per lane, not
    // a pair per walk, or they are two classes sharing the word.
    for c in 0..RATE {
        let mut k: Class = alloc::vec![Cell {
            row: word(NOTE_ROOT + c),
            col: 0
        }];
        for &m in l.member.iter() {
            k.push(Cell {
                row: m + l.depth * rounds,
                col: c,
            });
        }
        g.push(k);
    }

    // Each retired nullifier is the one its key hierarchy produced.
    for (i, &base) in l.key.iter().enumerate() {
        let nf = base + 3 * l.key_span[i] + rounds;
        let at = if i == 0 { NF0 } else { NF1 };
        for c in 0..RATE {
            g.push(pair(word(at + c), 0, nf, c));
        }
    }

    // Each created commitment is the one the output note committed to.
    for (i, at) in [OUT_CM0, OUT_CM1].into_iter().enumerate() {
        let cm = l.note[2 + i] + 2 * l.span_op + rounds;
        for c in 0..RATE {
            g.push(pair(word(at + c), 0, cm, c));
        }
    }

    // The public legs are the recomposed values the balance summed, not numbers
    // restated beside it.
    g.push(pair(word(PUBLIC_AMOUNT), 0, l.balance + 4, 3));
    g.push(pair(word(FEE), 0, l.balance + 5, 3));
    // Limb two is a note's asset. Conservation sums values and knows nothing of
    // assets, so the four notes hold theirs equal or value crosses between them.
    // A swap is not one asset in and the same out, and will need its own sum.
    const ASSET_LIMB: usize = 2;
    let mut assets: Class = alloc::vec![Cell {
        row: word(ASSET_ID),
        col: 0
    }];
    for &n in l.note.iter() {
        assets.push(Cell {
            row: n,
            col: ASSET_LIMB,
        });
    }
    g.push(assets);

    // Same for the association list: every walk ends at the published root, not
    // just the first one's.
    for c in 0..RATE {
        let mut k: Class = alloc::vec![Cell {
            row: word(ASSOC_ROOT + c),
            col: 0
        }];
        for &a in l.assoc.iter() {
            k.push(Cell {
                row: a + l.depth * rounds,
                col: c,
            });
        }
        g.push(k);
    }
    g
}
