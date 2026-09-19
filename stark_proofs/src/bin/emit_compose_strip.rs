// NONOS Operating System (AGPL-3.0-or-later)
//! The compose region's strip binding: which mode it is in, where the
//! accumulator cells sit, and the linear statement behind every out pin.
//!
//! At settlement the compose region does not recompute the inner's transitions.
//! It takes the strip branch, where each out pin is a three term residual
//!
//!     out[i] - acc[i] - stmt[i]
//!
//! with the multiplicative part computed in the accumulator strip and arriving
//! in the acc cells, and `stmt[i]` only a linear form over the region's own
//! window columns. A consumer implementing the other branch recomputes the
//! inner from the frame, gets plausible values, and disagrees on exactly those
//! lanes with nothing else disturbed.
//!
//! None of this was emitted. `strip_n_out` was, which is worse than nothing
//! being emitted, because it reads as the count of inner transitions and is
//! not: the inner has 16 transitions and the strip has 32 outputs, one per base
//! lane of the extension. A reader with every published field can still infer
//! the wrong structure from them, which is the failure class this repository
//! already had a name for and had not applied to itself.
//!
//! This is AIR data, not proof data: a pure function of the inner's constraint
//! code by way of the recorded tape. It costs an inner proof and a plan, not an
//! assembly and not a Keccak.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::recursion_assembly::{compose_step, inner, strip};
use std::time::Instant;

fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "compose-strip.json".into());

    let h = inner::hasher();
    let t0 = Instant::now();
    let inner_proof = inner::shield_join_split(&h);
    eprintln!("inner in {:?}", t0.elapsed());

    let w = inner_proof.proof.ood_frame.len();
    let p = inner_proof.ci.periodic_z.len();
    let nt = Air::num_transition(&inner_proof.air);
    let b = Air::boundary(&inner_proof.air).len();
    let k = Air::log_trace_len(&inner_proof.air) as usize;

    let t1 = Instant::now();
    let ss = strip::strip_side(&inner_proof);
    eprintln!(
        "strip plan in {:?}, {} statements",
        t1.elapsed(),
        ss.stmt.len()
    );

    let stmt = ss.stmt.clone();
    let (region, _trace) = compose_step::compose_gen_region_strip(inner_proof, stmt);

    /*
     * The acc cells come from the region itself rather than from the slot
     * formula repeated here. The formula is in one place on purpose and a
     * second copy is how an emit starts describing a layout the engine no
     * longer has.
     */
    let acc_cols: Vec<usize> = (0..ss.n_out / 2).map(|i| region.acc_col(i)).collect();
    let acc_slots: Vec<usize> = acc_cols.iter().map(|c| c / 2).collect();

    /*
     * Each statement is one base lane: two lanes per inner transition, c0 then
     * c1. `input_coeffs` are (base window column, coefficient) pairs, where the
     * column addresses the compose region's own window: lanes 0..2w are the
     * inner frame as interleaved c0/c1, lanes 2w..2(w+p) the periodic values.
     */
    let stmts: Vec<String> = ss
        .stmt
        .iter()
        .enumerate()
        .map(|(j, s)| {
            let pairs: Vec<String> = s
                .input_coeffs
                .iter()
                .map(|(u, c)| format!("[{u}, {}]", c.to_u64()))
                .collect();
            format!(
                "{{\"lane\": {j}, \"out\": {}, \"part\": \"{}\", \
                 \"constant\": {}, \"n_terms\": {}, \"input_coeffs\": [{}]}}",
                j / 2,
                if j % 2 == 0 { "c0" } else { "c1" },
                s.constant.to_u64(),
                s.input_coeffs.len(),
                pairs.join(", ")
            )
        })
        .collect();

    let json = format!(
        "{{\n  \"point\": \"settlement\",\n  \"compose_mode\": \"strip\",\n  \
         \"out_pin\": \"out[i] - acc[i] - stmt[i]\",\n  \
         \"inner_frame_slots\": {w},\n  \"inner_periodic_slots\": {p},\n  \
         \"inner_num_transition\": {nt},\n  \"inner_num_boundary\": {b},\n  \
         \"inner_log_trace_len\": {k},\n  \
         \"strip_n_out_lanes\": {},\n  \"out_cells\": {},\n  \
         \"acc_base_slot\": {},\n  \"acc_slots\": {:?},\n  \"acc_base_cols\": {:?},\n  \
         \"statements\": [\n    {}\n  ]\n}}\n",
        ss.n_out,
        ss.n_out / 2,
        acc_slots.first().copied().unwrap_or(0),
        acc_slots,
        acc_cols,
        stmts.join(",\n    "),
    );
    std::fs::write(&out, &json).expect("write the compose strip binding");
    eprintln!(
        "wrote {out}: acc base slot {}, {} statements",
        acc_slots.first().copied().unwrap_or(0),
        ss.stmt.len()
    );
}
