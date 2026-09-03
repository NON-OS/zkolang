// NONOS Operating System (AGPL-3.0-or-later)
//! The real outer's shape, from the full thirty-two query assembly: the
//! structure a settlement verifier derives its constants from, the baked
//! periodic root it holds, and a satisfaction walk over the whole witness so
//! the shape shipped is a shape that provably accepts. No proving here; the
//! proof artifact is the emitter's job. This is the contract side's re-gate
//! input for the real circuit, produced whole or not at all.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::recursion_assembly::{assemble_real, inner, Tamper};
use std::time::Instant;

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "real-structure.json".into());

    let t0 = Instant::now();
    let asm = assemble_real(Tamper::None);
    eprintln!("assembled in {:?}", t0.elapsed());

    let t1 = Instant::now();
    let ok = stark_proofs::witness_satisfies_public(&asm.wired, &asm.witness);
    eprintln!("satisfies in {:?}: {ok}", t1.elapsed());
    if !ok {
        eprintln!("the full-coverage assembly does not satisfy; refusing to emit its shape");
        std::process::exit(1);
    }

    let h = inner::hasher();
    let js = stark_proofs::shield_deployed_wired();
    let root = stark_proofs::crypto::stark::air::periodic_root_poseidon(&js, inner::extra(), &h);
    let root_hex: String = root
        .iter()
        .map(|l| format!("{:016x}", l.to_u64()))
        .collect::<Vec<_>>()
        .join("");

    let json = format!(
        "{{\n  \"log_trace_len\": {},\n  \"trace_width\": {},\n  \"num_transition\": {},\n  \
         \"num_groups\": {},\n  \"constraint_degree\": {},\n  \"inner_log_trace_len\": {},\n  \
         \"inner_trace_width\": {},\n  \"n_queries\": {},\n  \"periodic_root_poseidon\": \"{}\"\n}}\n",
        asm.wired.log_trace_len(),
        asm.wired.trace_width(),
        asm.wired.num_transition(),
        asm.n_groups,
        asm.wired.constraint_degree(),
        asm.lay.t_inner.trailing_zeros(),
        asm.lay.width_inner,
        asm.lay.n_q,
        root_hex,
    );
    std::fs::write(&out, &json).expect("write structure");
    println!("{json}");
    println!("wrote {out}");

    // The region layout, for a verifier whose evaluators are config-driven
    // over it: every Layout scalar, the region bases, and the permutation's
    // committed geometry. Same discipline as the shape file: derived, gated
    // by the satisfaction walk above, never typed.
    let lay = &asm.lay;
    let (sel_idx, row_idx, sig_base) = asm.wired.permutation_columns();
    let groups_json: Vec<String> = asm
        .wired
        .group_params()
        .iter()
        .zip(&sig_base)
        .map(|((cols, beta, gamma), sb)| {
            format!(
                "{{\"wired_cols\": {:?}, \"beta\": {}, \"gamma\": {}, \"sigma_base_col\": {}}}",
                cols,
                beta.to_u64(),
                gamma.to_u64(),
                sb
            )
        })
        .collect();
    let layout = format!(
        "{{\n  \"span\": {},\n  \"l\": {},\n  \"n_q\": {},\n  \"region_offsets\": {:?},\n  \
         \"z_op\": {},\n  \"claim_op\": {},\n  \"deep_coeff_op\": {},\n  \"pub_len\": {},\n  \
         \"ntr\": {},\n  \"ncoeff2\": {},\n  \"n_terms\": {},\n  \"width_inner\": {},\n  \
         \"window_inner\": {},\n  \"depth\": {},\n  \"n_open\": {},\n  \"n_folds\": {},\n  \
         \"log_n_inner\": {},\n  \"pbits\": {},\n  \"fbits\": {},\n  \"t_inner\": {},\n  \
         \"n_pz\": {},\n  \"pa_depth\": {},\n  \"n_chunks\": {},\n  \"frame_len\": {},\n  \
         \"n_coeff\": {},\n  \"c_periodic_col\": {},\n  \"c_z_col\": {},\n  \"c_coeff_col\": {},\n  \
         \"c_comp_z_col\": {},\n  \"sel_col\": {},\n  \"row_col\": {},\n  \"outer_periodic_root_keccak\": \"{}\",\n  \
         \"groups\": [\n    {}\n  ]\n}}\n",
        lay.span,
        lay.l,
        lay.n_q,
        asm.region_offsets,
        lay.z_op,
        lay.claim_op,
        lay.deep_coeff_op,
        lay.pub_len,
        lay.ntr,
        lay.ncoeff2,
        lay.n_terms,
        lay.width_inner,
        lay.window_inner,
        lay.depth,
        lay.n_open,
        lay.n_folds,
        lay.log_n,
        lay.pbits,
        lay.fbits,
        lay.t_inner,
        lay.n_pz,
        lay.pa_depth,
        lay.n_chunks,
        lay.frame_len,
        lay.n_coeff,
        lay.c_periodic_col,
        lay.c_z_col,
        lay.c_coeff_col,
        lay.c_comp_z_col,
        sel_idx,
        row_idx,
        {
            let r = stark_proofs::crypto::stark::air::periodic_root(&asm.wired, 0);
            r.iter().map(|b| format!("{b:02x}")).collect::<String>()
        },
        groups_json.join(",\n    "),
    );
    let lay_out = out.replace(".json", "-layout.json");
    std::fs::write(&lay_out, &layout).expect("write layout");
    println!("wrote {lay_out}");
}
