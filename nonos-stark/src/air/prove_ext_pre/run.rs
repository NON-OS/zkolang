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

//! The preprocessed prover driver: the same transcript walk as the plain path, but the
//! periodic schedule enters as a baked root rather than a recomputed region. It absorbs the
//! claimed periodic values after the frame, widens the DEEP coefficient draw to cover one
//! quotient per periodic column, and opens the sidecar row per query. The schedule bake is the
//! half of the settlement circuit this deletes, so a proof carries claims and opened rows in
//! place of the recompute.

use super::super::super::field::Fp2;
use super::super::super::fri_ext::fri_prove_ext;
use super::super::super::merkle::MerkleTree;
use super::super::super::transcript::Transcript;
use super::super::composition::num_coeffs;
use super::super::periodic_root::periodic_tree_over;
use super::super::prove_ext::{
    comp_at_z, draw_ood_point_ext, ood_frame, over_domain, trace_coeffs, wide_streamed,
    Domain,
};
use crate::poly::eval_coeff_cols_at_ext;
use super::super::spec::AirExt;
use super::super::types_ext::StarkProofExt;
use super::super::types_ext_pre::StarkProofExtPre;
use super::{deep, queries};
use crate::field::Fp;
use alloc::vec::Vec;

/*
 * Phase timing, so a long run says where it is instead of printing nothing
 * between its first line and its last. A settlement proof runs for hours and
 * the only signal used to be that the process was still alive, which is the
 * difference between waiting and being blind. Present only under the parallel
 * feature, which is the build that has a standard library to print with.
 */
struct Phase {
    #[cfg(feature = "parallel")]
    at: std::time::Instant,
}

impl Phase {
    fn start() -> Phase {
        Phase {
            #[cfg(feature = "parallel")]
            at: std::time::Instant::now(),
        }
    }

    #[allow(unused_variables)]
    fn done(&mut self, what: &str) {
        #[cfg(feature = "parallel")]
        {
            let now = std::time::Instant::now();
            std::eprintln!(
                "[prove] {what} in {:?}, resident {}",
                now.duration_since(self.at),
                resident()
            );
            self.at = now;
        }
    }
}

/*
 * What the process is holding, read from the kernel rather than estimated.
 * Every guess at this number tonight was wrong by a factor, and a phase that
 * reports its own peak turns the next failure into one line instead of another
 * evening. Linux only and best effort: a platform without the file says so.
 */
#[cfg(feature = "parallel")]
fn resident() -> alloc::string::String {
    use alloc::string::ToString;
    match std::fs::read_to_string("/proc/self/statm") {
        Ok(s) => {
            let pages: u64 = s.split_whitespace().nth(1).and_then(|v| v.parse().ok()).unwrap_or(0);
            alloc::format!("{:.1} GB", pages as f64 * 4096.0 / 1e9)
        }
        Err(_) => "unknown".to_string(),
    }
}

/// Prove `trace` against `air` with the periodic sidecar, at the given FRI
/// rate. The verifier must hold the matching baked periodic root.
///
/// The transcript order below is the protocol and matches the materialized
/// prover this replaced; the periodic tree comes through the same helper a
/// registered root does, so the two are one object by construction. Nothing is
/// held over the full domain but the two Fp2 codewords and the leaf digests.
pub fn stark_prove_ext_preprocessed<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> StarkProofExtPre {
    let d = Domain::of(air, extra_blowup_bits);
    let mut phase = Phase::start();

    let mut transcript = Transcript::new(b"NONOS-STARK-EXT");
    let tc = trace_coeffs(trace, &d);
    let trace_tree = wide_streamed(&tc, &d);
    let trace_root = trace_tree.root();
    transcript.absorb_digest(&trace_root);
    phase.done("trace commitment");

    let coeffs: Vec<Fp2> = (0..num_coeffs(air))
        .map(|_| transcript.challenge_fp2())
        .collect();

    /*
     * The columns are built once and handed to the committer, which drops them
     * as soon as it has interpolated them. Building them here and again inside
     * the committer paid for the construction twice and held two copies of a
     * set that is gigabytes wide on the settlement outer.
     *
     * Everything below wanted them for two things: the count, and the value at
     * the out-of-domain point. The count is taken before the handover and the
     * value comes off the coefficients, which are the same polynomials.
     */
    let (pc, p_tree) = periodic_tree_over(air.periodic_columns(), &d);
    let n_periodic = pc.len();
    phase.done("periodic commitment");
    let comp_d = over_domain(air, &d, &tc, &pc, &coeffs);
    let comp_tree = MerkleTree::commit_ext(&comp_d);
    transcript.absorb_digest(&comp_tree.root());
    phase.done("composition");

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
    let comp_z = comp_at_z(air, &d, &frame, &periodic_z, z, &coeffs);
    phase.done("out of domain");

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
    phase.done("deep and fri");
    let deep_tree = MerkleTree::commit_ext(&deep_d);
    transcript.absorb_digest(&fri.roots[0]);

    let (qs, openings) = queries::open(
        &mut transcript,
        n_queries,
        &d,
        &tc,
        &trace_tree,
        &pc,
        &p_tree,
        &comp_d,
        &comp_tree,
        &deep_d,
        &deep_tree,
    );
    phase.done("query openings");
    StarkProofExtPre {
        proof: StarkProofExt {
            trace_root,
            comp_root: comp_tree.root(),
            ood_frame: frame,
            fri,
            queries: qs,
        },
        periodic_z,
        openings,
    }
}
