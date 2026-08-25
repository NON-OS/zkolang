// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Air, Poseidon, TranscriptCheck, TranscriptOp, RATE, WIDTH};
use crate::crypto::stark::field::Fp;
use crate::recursion_assembly::inner::LOG_ROUNDS;
use crate::recursion_assembly::sponge::{absorb, squeeze};
use crate::shield::agg::{absorbed_at, read_effect};
use crate::shield::join::publics::{Intent, NF0, OUT_CM0};
use alloc::vec::Vec;

fn h() -> Poseidon {
    Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE])
}

fn d(x: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    for (i, lane) in k.iter_mut().enumerate() {
        *lane = Fp::from_u64(x * 16 + i as u64);
    }
    k
}

fn intent() -> Intent {
    Intent {
        note_root: d(1),
        assoc_root: d(2),
        nf: [d(10), d(11)],
        out_cm: [d(20), d(21)],
        public_amount: 0,
        fee: 3,
        asset_id: 7,
        clearing_price: 0,
        recipient: 0x1234,
    }
}

/// The publics-first prefix of the real transcript, built the way
/// `stark_transcript` builds it: one absorb per public word, before anything
/// else rides the sponge.
fn region(words: &[Fp], squeezes: usize) -> (Vec<Fp>, usize, usize) {
    let (h, mut st, mut ops) = (h(), [Fp::ZERO; WIDTH], Vec::<TranscriptOp>::new());
    for &w in words {
        absorb(&h, &mut ops, &mut st, w);
    }
    for _ in 0..squeezes {
        squeeze(&h, &mut ops, &mut st);
    }
    let r = TranscriptCheck::new_witness(h, LOG_ROUNDS, ops);
    (r.trace(), r.trace_width(), 1usize << LOG_ROUNDS)
}

/// The accept. What the read returns is what the transcript absorbed.
#[test]
fn the_read_returns_the_words_the_transcript_absorbed() {
    let i = intent();
    let (t, w, l) = region(&i.words(), 0);
    let e = read_effect(&t, w, l, 0);
    assert!(e.nullifiers == i.nf && e.outputs == i.out_cm);
}

/// The value rides the injection column. A state lane at the same row holds the
/// sponge mid flight, which moves with the publics without being them.
#[test]
fn a_state_lane_at_the_absorb_row_is_not_the_public() {
    let i = intent();
    let (t, w, l) = region(&i.words(), 0);
    let (row, col) = absorbed_at(l, NF0);
    assert_eq!(t[row * w + col], i.nf[0][0]);
    assert_ne!(
        t[row * w],
        i.nf[0][0],
        "a state lane read as the public word"
    );
}

/// Off by one word is a shifted fiction: still a real cell, still bound, and a
/// different move than the proof made.
#[test]
fn reading_one_word_over_composes_a_different_effect() {
    let i = intent();
    let (t, w, l) = region(&i.words(), 0);
    let shifted = read_effect(&t, w, l, 1);
    assert!(shifted.nullifiers != i.nf || shifted.outputs != i.out_cm);
}

/// The cell tracks the proof: publish a different note and the cell moves with
/// it, so the read cannot be pinned to a value the node preferred.
#[test]
fn changing_a_published_note_moves_the_cell() {
    let (mut a, mut b) = (intent(), intent());
    b.out_cm[0] = d(99);
    a.out_cm[0] = d(20);
    let (ta, w, l) = region(&a.words(), 0);
    let (tb, _, _) = region(&b.words(), 0);
    let (row, col) = absorbed_at(l, OUT_CM0);
    assert_ne!(ta[row * w + col], tb[row * w + col]);
}

/// The cell math against the real assembled trace rather than a rebuild of its
/// prefix. The transcript is region zero at offset zero, which is why the
/// statement bindings address its cells with no offset either. Ignored with the
/// rest of the assembly gate: it proves the whole inner proof to get the trace.
#[test]
#[ignore]
fn every_public_of_the_real_assembly_is_where_the_read_looks() {
    use crate::recursion_assembly::{assemble, Tamper};
    let asm = assemble(Tamper::None);
    let w = asm.wired.trace_width();
    for (i, p) in asm.publics.iter().enumerate() {
        let (row, col) = absorbed_at(asm.lay.l, i);
        assert_eq!(
            asm.witness[row * w + col],
            *p,
            "public {i} is not at the read's cell"
        );
    }
}

/// What makes the cell bound rather than merely located: the publics ride the
/// sponge, so a proof carrying different ones squeezes different challenges and
/// is a different transcript. Swapping the cells breaks the verify above them.
#[test]
fn different_publics_squeeze_a_different_challenge() {
    let (a, mut b) = (intent(), intent());
    b.nf[1] = d(77);
    let n = a.words().len();
    let (ta, w, l) = region(&a.words(), 2);
    let (tb, _, _) = region(&b.words(), 2);
    assert_ne!(
        ta[n * l * w],
        tb[n * l * w],
        "the transcript did not absorb the publics"
    );
}
