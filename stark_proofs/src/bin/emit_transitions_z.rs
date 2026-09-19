// NONOS Operating System (AGPL-3.0-or-later)
//! An outer AIR's transition vector at the out-of-domain point, and the
//! composition the proof determines there.
//!
//! This is the per lane gate the frozen point had and settlement did not. With
//! the 389 values in hand a consumer diffs lane by lane and the disagreement
//! names the kind; without them a wrong body is one wrong number at the end of
//! a sum of 1,579 terms, and bisecting that is guesswork.
//!
//! It costs an assembly and one evaluation of the AIR. Nothing is proved and
//! nothing is hashed: the frame at z and the periodic values at z are already
//! in the artifact, so this reads them out and calls the same
//! `transition_ext` the prover and the verifier both call.
//!
//! `comp_z` is emitted beside them. It is the one value the chain verifier is
//! still handed rather than derives, and the contracts side recovered it for
//! settlement by solving the DEEP identity at one query and closing the other
//! thirty one. That recovery stands; this prints the same value from the
//! transcript's own prefix, replayed at the proof's rate, so a walk over a new
//! artifact starts from a number two independent derivations agree on rather
//! than from one.
//!
//! The inputs are echoed back alongside the outputs on purpose. If the
//! consumer's decode of the frame or of the sidecar claims differs from this
//! one, the transition values would differ for a reason that has nothing to do
//! with a body being wrong, and the two sides need to rule that out before
//! reading anything into a lane diff.
//!
//! The point comes from the selector: settlement by default, `transfer` with
//! `queries=N` for the recursion over a transfer inner. The artifact must be
//! the one proved over that outer, and the frame width check below is what
//! says so.

use stark_proofs::crypto::stark::air::replay_pre::replay_comp_z_pre;
use stark_proofs::crypto::stark::air::{
    deserialize_proof_ext, serialize_proof_ext, Air, AirExt, StarkProofExtPre,
};
use stark_proofs::crypto::stark::field::{Fp, Fp2};
use stark_proofs::recursion_assembly::point::Point;
use stark_proofs::shield_params::deployment;
use std::time::Instant;

/// Read the sidecar's claims at z, which follow the base proof encoding.
///
/// The offset is recovered by re-serialising the parsed base rather than by
/// assuming a length: the base encoding is frozen but it is frozen somewhere
/// else, and a constant here would be a second opinion about it.
fn claims_at_z(bytes: &[u8], base_len: usize) -> Vec<Fp2> {
    let n = u32::from_le_bytes(bytes[base_len..base_len + 4].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(n);
    let mut p = base_len + 4;
    for _ in 0..n {
        let c0 = u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap());
        let c1 = u64::from_le_bytes(bytes[p + 8..p + 16].try_into().unwrap());
        out.push(Fp2 {
            c0: Fp::from_u64(c0),
            c1: Fp::from_u64(c1),
        });
        p += 16;
    }
    out
}

fn f2(v: &Fp2) -> String {
    format!("[{}, {}]", v.c0.to_u64(), v.c1.to_u64())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let point = Point::from_args(&args);
    /*
     * `unbound` replays the transcript with no public inputs absorbed, for an
     * artifact proved before the binding existed. Such a proof is a
     * measurement of the prover and not a proof about a batch, and its
     * comp_z is the one under the transcript it was actually made with.
     */
    let unbound = args.iter().any(|a| a == "unbound");
    /*
     * Two positional arguments, the artifact and the output, in that order,
     * after the point's own flags are set aside.
     */
    let mut positional = args
        .iter()
        .filter(|a| !Point::is_flag(a) && a.as_str() != "unbound");
    let proof_path = positional
        .next()
        .cloned()
        .unwrap_or_else(|| format!("emissions/{}-pre.proof", point.name()));
    let out = positional
        .next()
        .cloned()
        .unwrap_or_else(|| "transitions-z.json".into());

    let bytes = std::fs::read(&proof_path).expect("read the artifact");
    let proof = deserialize_proof_ext(&bytes).expect("the base half must parse");
    let base_len = serialize_proof_ext(&proof).len();
    let periodic_z = claims_at_z(&bytes, base_len);
    let frame = &proof.ood_frame;
    eprintln!(
        "artifact  {} bytes, base {} bytes, frame {} cells, claims {}",
        bytes.len(),
        base_len,
        frame.len(),
        periodic_z.len()
    );

    let t0 = Instant::now();
    let asm = match point.assemble() {
        Ok(asm) => asm,
        Err(why) => {
            eprintln!("{why}");
            std::process::exit(2);
        }
    };
    eprintln!("assembled {} in {:?}", point.name(), t0.elapsed());

    let width = Air::trace_width(&asm.wired);
    let window = Air::window_size(&asm.wired);
    if frame.len() != width * window {
        eprintln!(
            "the frame is {} cells but the AIR is {} wide over a {} row window; \
             these are not the same circuit",
            frame.len(),
            width,
            window
        );
        std::process::exit(1);
    }

    /*
     * The same call the prover makes inside comp_at_z and the verifier makes
     * when it recomputes the composition. Nothing here is a reimplementation:
     * if this disagrees with a consumer, the consumer's bodies disagree with
     * the engine's.
     */
    let t1 = Instant::now();
    let transitions = AirExt::transition_ext(&asm.wired, frame, &periodic_z);
    eprintln!(
        "evaluated {} transitions in {:?}",
        transitions.len(),
        t1.elapsed()
    );

    /*
     * The composition at z, from the transcript prefix replayed at the outer's
     * own rate. The openings are not needed for this and are not parsed; the
     * base proof and the claims are the whole input. A consumer holding a
     * different comp_z for this artifact is decoding the transcript
     * differently, and that is the disagreement to settle before any lane.
     */
    let pre = StarkProofExtPre {
        proof,
        periodic_z,
        openings: Vec::new(),
    };
    /*
     * Under the assembly's own public inputs, which the prover absorbed first
     * of all. A replay without them derives a different z, and its comp_z is
     * the composition at a point the proof never opened.
     */
    let absorbed: &[Fp] = if unbound { &[] } else { &asm.publics };
    let replayed = replay_comp_z_pre(&asm.wired, &pre, deployment::EXTRA_BLOWUP_BITS, absorbed);
    eprintln!(
        "comp_z    c0 {} c1 {} (z c0 {} c1 {}, {} coefficients, {} publics absorbed)",
        replayed.comp_z.c0.to_u64(),
        replayed.comp_z.c1.to_u64(),
        replayed.z.c0.to_u64(),
        replayed.z.c1.to_u64(),
        replayed.coeffs.len(),
        absorbed.len()
    );
    let publics_s: Vec<String> = absorbed.iter().map(|v| v.to_u64().to_string()).collect();
    let frame = &pre.proof.ood_frame;
    let periodic_z = &pre.periodic_z;

    let values: Vec<String> = transitions.iter().map(f2).collect();
    let claims: Vec<String> = periodic_z.iter().map(f2).collect();
    let frame_s: Vec<String> = frame.iter().map(f2).collect();

    /*
     * Where the lanes divide, so a diff can be read without holding the emit
     * open beside it: region transitions overlay from zero and the groups
     * follow them, one lane each.
     */
    let n_groups = Air::num_transition(&asm.wired) - region_lanes(&asm);
    let json = format!(
        "{{\n  \"point\": \"{}\",\n  \"artifact\": \"{}\",\n  \
         \"trace_width\": {},\n  \"window_size\": {},\n  \
         \"log_trace_len\": {},\n  \"num_transition\": {},\n  \
         \"region_lanes\": {},\n  \"group_lanes\": {},\n  \
         \"n_claims\": {},\n  \"n_frame_cells\": {},\n  \
         \"extra_blowup_bits\": {},\n  \"n_coeffs\": {},\n  \
         \"n_publics\": {},\n  \"publics\": [{}],\n  \
         \"z\": {},\n  \"comp_z\": {},\n  \
         \"transitions_z\": [\n    {}\n  ],\n  \
         \"ood_frame\": [\n    {}\n  ],\n  \
         \"periodic_z\": [\n    {}\n  ]\n}}\n",
        point.name(),
        proof_path,
        width,
        window,
        Air::log_trace_len(&asm.wired),
        Air::num_transition(&asm.wired),
        region_lanes(&asm),
        n_groups,
        periodic_z.len(),
        frame.len(),
        deployment::EXTRA_BLOWUP_BITS,
        replayed.coeffs.len(),
        absorbed.len(),
        publics_s.join(", "),
        f2(&replayed.z),
        f2(&replayed.comp_z),
        values.join(",\n    "),
        frame_s.join(",\n    "),
        claims.join(",\n    "),
    );
    std::fs::write(&out, &json).expect("write the transition vector");
    eprintln!("wrote {out}, {} lanes", transitions.len());
}

/// The number of lanes the region kinds overlay into, which is the maximum
/// arity over the kinds rather than their sum.
fn region_lanes(asm: &stark_proofs::recursion_assembly::Assembly) -> usize {
    asm.wired
        .kind_map()
        .iter()
        .map(|&(_, _, arity, _, _)| arity)
        .max()
        .unwrap_or(0)
}
