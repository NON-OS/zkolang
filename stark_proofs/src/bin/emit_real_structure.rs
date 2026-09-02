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
}
