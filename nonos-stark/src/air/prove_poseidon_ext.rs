// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The Poseidon-committed money-grade DEEP STARK prover: the same construction as
//! `prove_ext`, but the trace, composition, and DEEP polynomial are committed with
//! Poseidon Merkle trees and the transcript is the Poseidon sponge, and the
//! low-degree test is the Poseidon money-grade FRI. Every step is then cheap to
//! re-verify inside a STARK, which is what makes a proof from here recursable.

use super::super::air::{Poseidon, RATE};
use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::fri_poseidon_ext::fri_prove_poseidon_ext;
use super::super::poly::{eval_cols_on_subgroup_ext, eval_ext, intt, lde, lde_from_coeffs};
use super::super::poseidon_merkle::{pack_base, pack_ext, PoseidonMerkleTree};
use super::super::poseidon_transcript::PoseidonTranscript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::draw_ood_poseidon::draw_ood_point_poseidon;
use super::spec::AirExt;
use super::types_poseidon_ext::{StarkProofExtP, StarkQueryExtP};
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Prove `trace` satisfies `air` at money-grade soundness, committed with Poseidon.
/// `extra_blowup_bits` sets the FRI rate exactly as the keccak prover.
pub fn stark_prove_poseidon_ext<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
) -> StarkProofExtP {
    stark_prove_poseidon_ext_pub(
        air,
        trace,
        n_queries,
        grind_bits,
        extra_blowup_bits,
        hasher,
        &[],
    )
}

/// The same prover, seeding the transcript with `publics` before the trace roots so
/// the proof is bound to those public inputs by Fiat-Shamir. A recursive verifier
/// replays the same seed, exposing the publics in its transcript column.
#[allow(clippy::too_many_arguments)]
pub fn stark_prove_poseidon_ext_pub<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
) -> StarkProofExtP {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, fri_log_blowup) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let blowup = 1usize << (log_n - log_t);
    let window_size = air.window_size();

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    let mut transcript = PoseidonTranscript::new(hasher.clone());
    for &p in publics {
        transcript.absorb(p);
    }
    let mut trace_d: Vec<Vec<Fp>> = Vec::with_capacity(width);
    let mut trace_coeffs: Vec<Vec<Fp>> = Vec::with_capacity(width);
    let mut trace_trees: Vec<PoseidonMerkleTree> = Vec::with_capacity(width);
    let mut trace_roots: Vec<[Fp; RATE]> = Vec::with_capacity(width);
    // A column's extension, its commitment and its interpolation depend on that
    // column alone, so the columns run together. The transcript still absorbs the
    // roots in column order below, which is what the verifier replays.
    let built: Vec<(Vec<Fp>, PoseidonMerkleTree, Vec<Fp>)> = crate::par::map_index(width, |c| {
        let column: Vec<Fp> = (0..t).map(|i| trace[i * width + c]).collect();
        let column_d = lde(&column, g, shift, omega, n);
        let leaves: Vec<[Fp; RATE]> = column_d.iter().map(|v| pack_base(*v)).collect();
        let tree = PoseidonMerkleTree::commit(hasher, &leaves);
        let coeffs = intt(&column, g);
        (column_d, tree, coeffs)
    });
    for (column_d, tree, coeffs) in built {
        transcript.absorb_digest(&tree.root());
        trace_roots.push(tree.root());
        trace_coeffs.push(coeffs);
        trace_trees.push(tree);
        trace_d.push(column_d);
    }

    let coeffs: Vec<Fp2> = (0..num_coeffs(air))
        .map(|_| transcript.challenge_fp2())
        .collect();

    let periodic_cols = air.periodic_columns();

    // The evaluation domain is `blowup` cosets of the order-t subgroup, and the
    // composition window steps by `blowup`, so a window never leaves the coset it
    // starts in. Extending the periodic columns one coset at a time holds
    // n_periodic * t values rather than n_periodic * n. For the recursion that is
    // the difference between a hundred and thirty gigabytes and half of one.
    let sub = omega.pow(blowup as u64);
    // Interpolate once. Each coset then costs one transform rather than a fresh
    // interpolation of the same column.
    let periodic_coeffs: Vec<Vec<Fp>> = periodic_cols.iter().map(|col| intt(col, g)).collect();
    let mut comp_d: Vec<Fp2> = alloc::vec![Fp2::ZERO; n];
    for c in 0..blowup {
        let shift_c = shift * omega.pow(c as u64);
        let periodic_c: Vec<Vec<Fp>> =
            crate::par::map_slice(&periodic_coeffs, |cf| lde_from_coeffs(cf, shift_c, sub, t));
        // Every constraint at every point of the domain, which for the
        // recursion is 33.5 million evaluations. Written as a recurrence over i
        // it runs on one core while the maps above it split a handful of cheap
        // extensions. The point at step i is shift_c * sub^i, so the steps are
        // independent; what is not free is the pair of buffers each needs, so
        // the work goes out in blocks. A block allocates once, walks its own
        // points, and carries its own x.
        const BLOCK: usize = 1024;
        let blocks = t.div_ceil(BLOCK);
        let parts = crate::par::map_index(blocks, |b| {
            let lo = b * BLOCK;
            let hi = (lo + BLOCK).min(t);
            let mut window: Vec<Fp2> = Vec::with_capacity(window_size * width);
            let mut periodic: Vec<Fp2> = Vec::with_capacity(periodic_c.len());
            let mut out: Vec<Fp2> = Vec::with_capacity(hi - lo);
            let mut x = shift_c * sub.pow(lo as u64);
            for i in lo..hi {
                let j = c + blowup * i;
                window.clear();
                for k in 0..window_size {
                    let idx = (j + k * blowup) % n;
                    for column in &trace_d {
                        window.push(Fp2::from_base(column[idx]));
                    }
                }
                periodic.clear();
                periodic.extend(periodic_c.iter().map(|pd| Fp2::from_base(pd[i])));
                out.push(compose_ext(
                    air,
                    g,
                    Fp2::from_base(x),
                    &window,
                    &periodic,
                    &coeffs,
                ));
                x = x * sub;
            }
            out
        });
        for (b, part) in parts.into_iter().enumerate() {
            for (k, v) in part.into_iter().enumerate() {
                comp_d[c + blowup * (b * BLOCK + k)] = v;
            }
        }
    }
    let comp_leaves: Vec<[Fp; RATE]> = comp_d.iter().map(|v| pack_ext(*v)).collect();
    let comp_tree = PoseidonMerkleTree::commit(hasher, &comp_leaves);
    transcript.absorb_digest(&comp_tree.root());

    let z = draw_ood_point_poseidon(&mut transcript, shift, n, t);
    let mut ood_frame: Vec<Fp2> = Vec::with_capacity(window_size * width);
    for k in 0..window_size {
        let zk = z * Fp2::from_base(g.pow(k as u64));
        for coeffs_c in &trace_coeffs {
            ood_frame.push(eval_ext(coeffs_c, zk));
        }
    }
    for value in &ood_frame {
        transcript.absorb(value.c0);
        transcript.absorb(value.c1);
    }

    let periodic_z: Vec<Fp2> = eval_cols_on_subgroup_ext(g, t, &periodic_cols, z);
    let comp_z = compose_ext(air, g, z, &ood_frame, &periodic_z, &coeffs);

    let deep_coeffs: Vec<Fp2> = (0..width * window_size + 1)
        .map(|_| transcript.challenge_fp2())
        .collect();

    let mut deep_d: Vec<Fp2> = Vec::with_capacity(n);
    let mut x = shift;
    for j in 0..n {
        let mut acc = Fp2::ZERO;
        for k in 0..window_size {
            let zk = z * Fp2::from_base(g.pow(k as u64));
            let inv_x_zk = (Fp2::from_base(x) - zk).inv();
            for (c, column) in trace_d.iter().enumerate() {
                let claimed = ood_frame[k * width + c];
                acc = acc
                    + deep_coeffs[k * width + c]
                        * ((Fp2::from_base(column[j]) - claimed) * inv_x_zk);
            }
        }
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((comp_d[j] - comp_z) * (Fp2::from_base(x) - z).inv());
        deep_d.push(acc);
        x = x * omega;
    }

    let fri = fri_prove_poseidon_ext(
        &deep_d,
        shift,
        fri_log_blowup,
        n_queries,
        grind_bits,
        hasher,
    );
    let deep_leaves: Vec<[Fp; RATE]> = deep_d.iter().map(|v| pack_ext(*v)).collect();
    let deep_tree = PoseidonMerkleTree::commit(hasher, &deep_leaves);
    transcript.absorb_digest(&fri.roots[0]);

    let mut queries: Vec<StarkQueryExtP> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let p = transcript.challenge_index(n);
        let trace_vals: Vec<Fp> = trace_d.iter().map(|col| col[p]).collect();
        let trace_paths: Vec<Vec<[Fp; RATE]>> =
            trace_trees.iter().map(|tree| tree.open(p)).collect();
        queries.push(StarkQueryExtP {
            deep: deep_d[p],
            deep_path: deep_tree.open(p),
            trace: trace_vals,
            trace_paths,
            comp: comp_d[p],
            comp_path: comp_tree.open(p),
        });
    }

    StarkProofExtP {
        trace_roots,
        comp_root: comp_tree.root(),
        ood_frame,
        fri,
        queries,
    }
}
