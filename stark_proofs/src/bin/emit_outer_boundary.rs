// NONOS Operating System (AGPL-3.0-or-later)
//! The outer's boundary constraints, and nothing else.
//!
//! The structure emit already carries these, but that run also commits the
//! outer periodic tree at the deployment rate, which is two thousand six
//! hundred and forty nine columns of Keccak and took three hours and thirty
//! eight minutes on the box. The boundary list does not depend on that tree in
//! any way, so waiting for it is waiting for nothing.
//!
//! This assembles the outer and stops. The assembly itself is minutes rather
//! than seconds, because the witness has to exist before the AIR knows its own
//! boundaries, but it is three orders of magnitude cheaper than the root and it
//! is the whole cost.
//!
//! What it buys, for a verifier holding a settlement proof: the boundary half
//! of the composition at the out-of-domain point stops being a prediction.
//! With `comp_z` pinned from the proof and the transition half computed, the
//! boundary half is already determined, but determined conditionally, because
//! a wrong transition family and a wrong boundary sum can be wrong by
//! compensating amounts and still agree. Computing the boundary independently
//! from the emitted triples is what collapses that into a result.
//!
//! Boundary order is the engine's own, which is the order the composition
//! coefficients were drawn against. A consumer pairs the two by index, and
//! neither side has to agree about sorting.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::crypto::stark::fri::root_of_unity;
use stark_proofs::recursion_assembly::{assemble_real, Tamper};
use std::time::Instant;

fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "outer-boundary.json".into());

    let t0 = Instant::now();
    let asm = assemble_real(Tamper::None);
    eprintln!("assembled settlement in {:?}", t0.elapsed());

    /*
     * The same gate the structure emit runs. A boundary list taken off an
     * assembly that does not satisfy its own constraints describes a circuit
     * nothing will ever prove, and it would be indistinguishable from a good
     * one on inspection.
     */
    let t1 = Instant::now();
    let ok = stark_proofs::witness_satisfies_public(&asm.wired, &asm.witness);
    eprintln!("satisfies in {:?}: {ok}", t1.elapsed());
    if !ok {
        eprintln!("the assembly does not satisfy; refusing to emit its boundaries");
        std::process::exit(1);
    }

    let boundary = Air::boundary(&asm.wired);
    let triples: Vec<String> = boundary
        .iter()
        .map(|(col, row, val)| format!("[{col}, {row}, {}]", val.to_u64()))
        .collect();

    /*
     * The trace domain points the boundary quotients divide by, emitted so the
     * chain never exponentiates. Every boundary term is
     * (frame[col] - expected) / (z - g^row), and the 1,190 triples sit on only
     * 228 distinct rows, two of which carry 510 of them. So a verifier that
     * computes g^row per triple does 1,190 exponentiations for 228 values that
     * are constants of the circuit. The prover's own ComposePlan learned this
     * once: evaluating them in the loop cost one exponentiation per boundary
     * per row. These are the same values, from the same generator, emitted
     * once. The exempt point g^(t-1) rides along for the same reason; it is
     * the one factor the transition quotients multiply back in.
     */
    let log_t = Air::log_trace_len(&asm.wired);
    let g = root_of_unity(log_t);
    let t = 1u64 << log_t;
    let mut rows: Vec<usize> = boundary.iter().map(|(_, row, _)| *row).collect();
    rows.sort_unstable();
    rows.dedup();
    let points: Vec<String> = rows
        .iter()
        .map(|&row| format!("[{row}, {}]", g.pow(row as u64).to_u64()))
        .collect();
    let exempt = g.pow(t - 1).to_u64();

    /*
     * The shape fields a consumer needs to use the list at all: the width and
     * the trace length bound the column and row indices, and the transition
     * count says how many coefficients precede the boundary ones in the draw.
     */
    let json = format!(
        "{{\n  \"point\": \"settlement\",\n  \"trace_width\": {},\n  \
         \"log_trace_len\": {},\n  \"num_transition\": {},\n  \
         \"num_boundary\": {},\n  \"constraint_degree\": {},\n  \
         \"window_size\": {},\n  \
         \"trace_generator\": {},\n  \"exempt_point\": {},\n  \
         \"n_distinct_rows\": {},\n  \
         \"boundary_points\": [\n    {}\n  ],\n  \
         \"outer_boundary\": [\n    {}\n  ]\n}}\n",
        Air::trace_width(&asm.wired),
        log_t,
        Air::num_transition(&asm.wired),
        boundary.len(),
        Air::constraint_degree(&asm.wired),
        Air::window_size(&asm.wired),
        g.to_u64(),
        exempt,
        rows.len(),
        points.join(",\n    "),
        triples.join(",\n    "),
    );
    std::fs::write(&out, &json).expect("write outer boundary");
    eprintln!(
        "wrote {out}, {} triples over {} distinct rows",
        boundary.len(),
        rows.len()
    );
}
