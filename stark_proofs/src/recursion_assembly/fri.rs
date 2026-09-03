// NONOS Operating System (AGPL-3.0-or-later)
//! Region 3 (the FRI transcript, shared across queries) and region 4 (the fold
//! chain, one per query). The transcript replays the root absorbs and beta
//! squeezes and draws every FRI query index; each fold carries the per-layer
//! points, inverses, and position bits the square-and-sign chain derives from its
//! query index `q_k`. Query 0 is `qs[0]`; closing coverage folds every `q_k`.

use super::inner::{Inner, GRIND, LOG_ROUNDS};
use super::sponge::{absorb, squeeze};
use super::tamper::Tamper;
use crate::crypto::stark::air::{
    AirExt, Poseidon, TraceFoldExt, TranscriptCheck, TranscriptOp, WIDTH,
};
use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
use alloc::vec::Vec;

/// The shared FRI transcript region and the per-query indices it draws.
pub struct FriTranscript {
    pub transcript: TranscriptCheck,
    pub ttrace: Vec<Fp>,
    pub betas: Vec<Fp2>,
    pub n_folds: usize,
    pub log_n: u32,
    /// Every FRI query index, drawn consecutively after the proof of work.
    pub qs: Vec<usize>,
}

pub fn fri_transcript<A: AirExt>(h: &Poseidon, inner: &Inner<A>) -> FriTranscript {
    let fri = &inner.proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;

    let mut fs = PoseidonTranscript::new(h.clone());
    let mut betas: Vec<Fp2> = Vec::with_capacity(n_folds);
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    for root in &fri.roots {
        fs.absorb_digest(root);
        betas.push(fs.challenge_fp2());
        for lane in root {
            absorb(h, &mut ops, &mut st, *lane);
        }
        squeeze(h, &mut ops, &mut st);
        squeeze(h, &mut ops, &mut st);
    }
    for value in &fri.final_layer {
        fs.absorb(value.c0);
        fs.absorb(value.c1);
    }
    assert!(
        fs.verify_pow(fri.pow_nonce, GRIND),
        "the FRI proof-of-work did not check"
    );
    // One index per FRI query, drawn in order; qs[k] is query k's fold position.
    let qs: Vec<usize> = (0..fri.queries.len())
        .map(|_| fs.challenge_index(n))
        .collect();
    let transcript = TranscriptCheck::new_witness(h.clone(), LOG_ROUNDS, ops);
    let ttrace = transcript.trace();

    FriTranscript {
        transcript,
        ttrace,
        betas,
        n_folds,
        log_n,
        qs,
    }
}

/// One query's fold chain (region 4), derived from its index `q_k`.
pub struct FoldSide {
    pub fold: TraceFoldExt,
    pub ftrace: Vec<Fp>,
    /// The layer-zero position of this query: `q_k mod (n / 2)`.
    pub ik: usize,
}

pub fn fri_fold_k<A: AirExt>(
    inner: &Inner<A>,
    ft: &FriTranscript,
    query: usize,
    tamper: Tamper,
) -> FoldSide {
    let fri = &inner.proof.fri;
    let n = 1usize << ft.log_n;
    let qk = ft.qs[query];
    let final_value = fri.final_layer[0];
    let bo = root_of_unity(ft.log_n);
    let shift = Fp::from_u64(7);
    let (mut a, mut b, mut xi, mut dir) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for (m, op) in fri.queries[query].layers.iter().enumerate() {
        a.push(op.a);
        b.push(op.b);
        let ix = qk % (n >> (m + 1));
        xi.push((shift * bo.pow(ix as u64)).pow(1u64 << m).inv());
        dir.push(ix >= (n >> (m + 2)));
    }
    a.push(final_value);
    b.push(final_value);
    let log_layers = (ft.n_folds + 1).next_power_of_two().trailing_zeros();
    let fold = TraceFoldExt::new_witness(log_layers, ft.n_folds, xi, dir, final_value);
    let betas = match tamper {
        Tamper::OffTranscriptBeta => {
            let mut v = ft.betas.clone();
            v[0] = v[0] + Fp2::ONE;
            v
        }
        _ => ft.betas.clone(),
    };
    let ftrace = fold.trace(&betas, &a, &b);
    FoldSide {
        fold,
        ftrace,
        ik: qk % (n >> 1),
    }
}

/// The query-0 combined form the current single-query `assemble()` consumes. Built
/// from the shared transcript plus query 0's fold, so its behavior is unchanged.
pub struct FriSide {
    pub transcript: TranscriptCheck,
    pub ttrace: Vec<Fp>,
    pub fold: TraceFoldExt,
    pub ftrace: Vec<Fp>,
    pub n_folds: usize,
    pub log_n: u32,
    /// The layer-zero position of query zero: q0 mod (n / 2).
    pub i0: usize,
}

pub fn fri_side<A: AirExt>(h: &Poseidon, inner: &Inner<A>, tamper: Tamper) -> FriSide {
    let ft = fri_transcript(h, inner);
    let f0 = fri_fold_k(inner, &ft, 0, tamper);
    FriSide {
        transcript: ft.transcript,
        ttrace: ft.ttrace,
        fold: f0.fold,
        ftrace: f0.ftrace,
        n_folds: ft.n_folds,
        log_n: ft.log_n,
        i0: f0.ik,
    }
}
