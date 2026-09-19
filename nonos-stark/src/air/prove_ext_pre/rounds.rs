// NONOS Operating System (AGPL-3.0-or-later)
//! The preprocessed prover in two commitment rounds.
//!
//! Identical to `run` except for where the permutation challenges come from.
//! The region columns are interpolated and committed, their root goes into the
//! transcript, beta and gamma come out of it, and only then are the
//! permutation columns filled and committed. Everything after that is the same
//! walk over a trace that now has two roots.
//!
//! This is the whole difference between a copy constraint that is argued and
//! one that is asserted. With the challenges fixed in the circuit the real
//! settlement outer accepts a witness in which two cells it says are equal are
//! not, which `wired_forgery_tests` builds.

use super::super::super::field::{Fp, Fp2};
use super::super::super::fri_ext::fri_prove_ext;
use super::super::super::merkle::MerkleTree;
use super::super::super::transcript::Transcript;
use super::super::composition::num_coeffs;
use super::super::periodic_root::periodic_tree_over;
use super::super::prove_ext::{
    comp_at_z, draw_ood_point_ext, ood_frame, over_domain, periodic_coeffs, trace_coeffs_cols,
    wide_streamed, Domain,
};
use super::super::rounds::Permuted;
use super::super::spec::AirExt;
use super::super::types_ext::StarkProofExt;
use super::super::types_ext_pre::StarkProofExtPre;
use super::super::types_ext_rounds::StarkProofExtRounds;
use super::{deep, queries};
use crate::poly::eval_coeff_cols_at_ext;
use alloc::vec::Vec;

/// Prove `air` over `trace`, drawing the permutation challenges between the two
/// commitments.
///
/// `trace` arrives with its region columns final and its permutation columns
/// unset; this fills them once the challenges exist. `air` is taken by value
/// and handed back with the drawn challenges in force, because a verifier has
/// to evaluate the same constraint set the composition was built from.
#[allow(clippy::too_many_arguments)]
pub fn stark_prove_ext_rounds<A: AirExt + Permuted>(
    mut air: A,
    trace: &mut [Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    publics: &[Fp],
    periodic_tree: Option<MerkleTree>,
) -> Option<(StarkProofExtRounds, MerkleTree, A)> {
    let d = Domain::of(&air, extra_blowup_bits);
    let rw = air.region_width();
    assert!(
        rw < d.width,
        "a two round proof needs permutation columns above the regions, got {rw} of {}",
        d.width
    );

    let mut transcript = Transcript::new(b"NONOS-STARK-EXT");
    for value in publics {
        transcript.absorb_fp(*value);
    }

    /*
     * Round one. The region columns are the prover's own witness and nothing
     * here depends on a challenge, so they commit first and their root is what
     * the challenges are drawn against.
     */
    let region_c = trace_coeffs_cols(trace, &d, 0, rw);
    let region_tree = wide_streamed(&region_c, &d);
    let region_root = region_tree.root();
    transcript.absorb_digest(&region_root);

    /*
     * The point the copy constraint is checked at, fixed now that the columns
     * it speaks about are. A prover that wants a particular beta has to find a
     * region trace that hashes to it.
     */
    let beta = transcript.challenge_fp();
    let gamma = transcript.challenge_fp();
    air.set_challenges(beta, gamma);
    air.fill_products(trace);

    // Round two.
    let perm_c = trace_coeffs_cols(trace, &d, rw, d.width);
    let perm_tree = wide_streamed(&perm_c, &d);
    let perm_root = perm_tree.root();
    transcript.absorb_digest(&perm_root);

    let mut tc = region_c;
    tc.extend(perm_c);

    let coeffs: Vec<Fp2> = (0..num_coeffs(&air))
        .map(|_| transcript.challenge_fp2())
        .collect();

    let (pc, p_tree) = match periodic_tree {
        Some(tree) if tree.len() == d.n => {
            let cols = air.periodic_columns();
            let pc = periodic_coeffs(&cols, &d);
            (pc, tree)
        }
        _ => periodic_tree_over(air.periodic_columns(), &d),
    };
    let n_periodic = pc.len();

    let comp_d = over_domain(&air, &d, &tc, &pc, &coeffs);
    let comp_tree = MerkleTree::commit_ext(&comp_d);
    transcript.absorb_digest(&comp_tree.root());

    let z = draw_ood_point_ext(&mut transcript, d.shift, d.n, d.t);
    let frame = ood_frame(&tc, &d, z);
    for value in &frame {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let periodic_z = eval_coeff_cols_at_ext(&pc, z);
    for value in &periodic_z {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let comp_z = comp_at_z(&air, &d, &frame, &periodic_z, z, &coeffs);

    let deep_coeffs: Vec<Fp2> = (0..d.width * d.window + 1 + n_periodic)
        .map(|_| transcript.challenge_fp2())
        .collect();
    let deep_d = deep::over_domain(
        &d,
        &tc,
        &pc,
        &comp_d,
        &frame,
        &periodic_z,
        comp_z,
        z,
        &deep_coeffs,
    );

    let fri = fri_prove_ext(&deep_d, d.shift, d.fri_log_blowup, n_queries, grind_bits);
    let deep_tree = MerkleTree::commit_ext(&deep_d);
    transcript.absorb_digest(&fri.roots[0]);

    let (qs, openings, perm_paths) = queries::open_rounds(
        &mut transcript,
        n_queries,
        &d,
        &tc,
        &region_tree,
        &perm_tree,
        &pc,
        &p_tree,
        &comp_d,
        &comp_tree,
        &deep_d,
        &deep_tree,
    );

    let rounds = StarkProofExtRounds {
        pre: StarkProofExtPre {
            proof: StarkProofExt {
                trace_root: region_root,
                comp_root: comp_tree.root(),
                ood_frame: frame,
                fri,
                queries: qs,
            },
            periodic_z,
            openings,
        },
        perm_root,
        region_width: rw,
        perm_paths,
    };
    Some((rounds, p_tree, air))
}
