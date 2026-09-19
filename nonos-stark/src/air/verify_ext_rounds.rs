// NONOS Operating System (AGPL-3.0-or-later)
//! Verifying a proof whose trace was committed in two rounds.
//!
//! The walk is the preprocessed verifier's, with two differences. The
//! permutation challenges are drawn from the first round's root and handed to
//! the AIR before anything is evaluated against it, and a query's row is
//! checked in halves, the regions under the first root and the permutation
//! columns under the second.
//!
//! Only these two things carry the soundness the fixed challenge form did not
//! have. A verifier that took beta and gamma from anywhere else, or that
//! checked one root against the whole row, would be back where it started.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::fri_ext::fri_verify_ext;
use super::super::merkle::{verify_path_ext, verify_path_wide, verify_path_wide_periodic};
use super::super::poly::eval_cols_on_subgroup_ext;
use super::super::transcript::Transcript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::prove_ext::draw_ood_point_ext;
use super::rounds::Permuted;
use super::spec::AirExt;
use super::types_ext_rounds::StarkProofExtRounds;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Verify `rounds` against `air` and the baked `periodic_root`.
///
/// `air` is taken by value and mutated with the drawn challenges, because the
/// constraint set the composition was built from is the one at those
/// challenges. A caller that reused an AIR carrying the circuit's defaults
/// would be checking a different statement.
#[allow(clippy::too_many_arguments)]
pub fn stark_verify_ext_rounds<A: AirExt + Permuted>(
    mut air: A,
    rounds: &StarkProofExtRounds,
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    periodic_root: &[u8; 32],
    publics: &[Fp],
) -> bool {
    let pre = &rounds.pre;
    let proof = &pre.proof;
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let rw = rounds.region_width;
    let (log_n, fri_log_blowup) = domain_params_blown(&air, extra_blowup_bits);
    let n = 1usize << log_n;
    let window_size = air.window_size();
    let periodic_cols = air.periodic_columns();
    let n_periodic = periodic_cols.len();

    /*
     * The split is the proof's claim and the AIR's fact, and a proof that
     * disagrees about it is checking two roots against halves of its own
     * choosing.
     */
    if rw != air.region_width() || rw >= width {
        return false;
    }
    if proof.ood_frame.len() != window_size * width
        || proof.queries.len() != n_queries
        || pre.periodic_z.len() != n_periodic
        || pre.openings.len() != n_queries
        || rounds.perm_paths.len() != n_queries
    {
        return false;
    }

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    let mut transcript = Transcript::new(b"NONOS-STARK-EXT");
    for value in publics {
        transcript.absorb_fp(*value);
    }
    transcript.absorb_digest(&proof.trace_root);
    let beta = transcript.challenge_fp();
    let gamma = transcript.challenge_fp();
    air.set_challenges(beta, gamma);
    transcript.absorb_digest(&rounds.perm_root);

    let coeffs: Vec<Fp2> = (0..num_coeffs(&air))
        .map(|_| transcript.challenge_fp2())
        .collect();
    transcript.absorb_digest(&proof.comp_root);
    let z = draw_ood_point_ext(&mut transcript, shift, n, t);
    for value in &proof.ood_frame {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    for value in &pre.periodic_z {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let deep_coeffs: Vec<Fp2> = (0..width * window_size + 1 + n_periodic)
        .map(|_| transcript.challenge_fp2())
        .collect();

    let ours = eval_cols_on_subgroup_ext(g, t, &periodic_cols, z);
    if ours != pre.periodic_z {
        return false;
    }
    let comp_z = compose_ext(&air, g, z, &proof.ood_frame, &pre.periodic_z, &coeffs);

    if !fri_verify_ext(
        &proof.fri,
        shift,
        log_n,
        fri_log_blowup,
        n_queries,
        grind_bits,
    ) {
        return false;
    }
    let deep_root = proof.fri.roots[0];
    transcript.absorb_digest(&deep_root);

    for ((qd, op), perm_path) in proof
        .queries
        .iter()
        .zip(pre.openings.iter())
        .zip(rounds.perm_paths.iter())
    {
        let p = transcript.challenge_index(n);
        if qd.trace.len() != width || op.row.len() != n_periodic {
            return false;
        }
        if !verify_path_ext(&deep_root, p, qd.deep, &qd.deep_path)
            || !verify_path_ext(&proof.comp_root, p, qd.comp, &qd.comp_path)
            || !verify_path_wide(&proof.trace_root, p, &qd.trace[..rw], &qd.trace_path)
            || !verify_path_wide(&rounds.perm_root, p, &qd.trace[rw..], perm_path)
            || !verify_path_wide_periodic(periodic_root, p, &op.row, &op.path)
        {
            return false;
        }

        let x = shift * omega.pow(p as u64);
        let mut acc = Fp2::ZERO;
        for k in 0..window_size {
            let zk = z * Fp2::from_base(g.pow(k as u64));
            let inv_x_zk = (Fp2::from_base(x) - zk).inv();
            for c in 0..width {
                let claimed = proof.ood_frame[k * width + c];
                acc = acc
                    + deep_coeffs[k * width + c]
                        * ((Fp2::from_base(qd.trace[c]) - claimed) * inv_x_zk);
            }
        }
        let inv_x_z = (Fp2::from_base(x) - z).inv();
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((qd.comp - comp_z) * inv_x_z);
        for (pi, v) in op.row.iter().enumerate() {
            let pc = deep_coeffs[width * window_size + 1 + pi];
            acc = acc + pc * ((Fp2::from_base(*v) - pre.periodic_z[pi]) * inv_x_z);
        }
        if acc != qd.deep {
            return false;
        }
    }

    true
}
