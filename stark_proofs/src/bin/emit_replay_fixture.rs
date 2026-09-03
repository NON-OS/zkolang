// NONOS Operating System (AGPL-3.0-or-later)
//! The transcript fixture for an independent verifier: every challenge the
//! frozen proof's transcript yields, emitted as known answers. The values are
//! trustworthy three ways before they are written: the proof verifies whole
//! from its bytes, the replay recomputes every query's DEEP value from its
//! own challenges, and the walk is the library's, not a copy. A ported
//! transcript that reproduces this file has reproduced the real walk.

use stark_proofs::crypto::stark::air::{deserialize_proof_ext, replay_ext, stark_verify_ext};
use stark_proofs::recursion_assembly::{assemble_real, Tamper};
use std::time::Instant;

const N_QUERIES: usize = stark_proofs::shield_params::dev::N_QUERIES;
const GRIND_BITS: u32 = stark_proofs::shield_params::dev::GRIND_BITS;

fn fp2_hex(v: &stark_proofs::crypto::stark::field::Fp2) -> String {
    format!("\"{:016x}{:016x}\"", v.c0.to_u64(), v.c1.to_u64())
}

fn main() {
    let proof_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "proofs/recursion-real.proof".into());
    let out = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "proofs/replay-fixture.json".into());

    let bytes = std::fs::read(&proof_path).expect("read proof");
    let proof = deserialize_proof_ext(&bytes).expect("parse proof");

    let t0 = Instant::now();
    let asm = assemble_real(Tamper::None);
    eprintln!("assembled in {:?}", t0.elapsed());

    let t1 = Instant::now();
    let ok = stark_verify_ext(&asm.wired, &proof, N_QUERIES, GRIND_BITS);
    eprintln!("verified in {:?}: {ok}", t1.elapsed());
    if !ok {
        eprintln!("the proof does not verify; refusing to emit a fixture from it");
        std::process::exit(1);
    }

    let r = replay_ext::replay_challenges_ext(&asm.wired, &proof, N_QUERIES);
    if !r.deep_consistent {
        eprintln!("the replay disagrees with the proof it just verified; walk drift");
        std::process::exit(1);
    }

    let coeffs: Vec<String> = r.coeffs.iter().map(fp2_hex).collect();
    let deep_coeffs: Vec<String> = r.deep_coeffs.iter().map(fp2_hex).collect();
    let json = format!(
        "{{\n  \"proof_len\": {},\n  \"n_queries\": {},\n  \"n_coeffs\": {},\n  \
         \"z\": {},\n  \"comp_z\": {},\n  \"indices\": {:?},\n  \
         \"coeffs\": [\n    {}\n  ],\n  \"deep_coeffs\": [\n    {}\n  ]\n}}\n",
        bytes.len(),
        N_QUERIES,
        r.coeffs.len(),
        fp2_hex(&r.z),
        fp2_hex(&r.comp_z),
        r.indices,
        coeffs.join(",\n    "),
        deep_coeffs.join(",\n    "),
    );
    std::fs::write(&out, &json).expect("write fixture");
    println!("wrote {out}");

    // The composition sibling: the periodic values at z the verifier computed
    // on the way to comp_z, and the outer boundary list verbatim in the
    // engine's own (column, row, value) order. Both from the emit that
    // produced comp_z, so a wrong array cannot reach the target value.
    use stark_proofs::crypto::stark::air::Air;
    let periodic_z: Vec<String> = r.periodic_z.iter().map(fp2_hex).collect();
    let boundary: Vec<String> = asm
        .wired
        .boundary()
        .iter()
        .map(|(col, row, v)| format!("[{}, {}, \"{:016x}\"]", col, row, v.to_u64()))
        .collect();
    let transitions_z: Vec<String> = r.transitions_z.iter().map(fp2_hex).collect();
    let comp = format!(
        "{{\n  \"comp_z\": {},\n  \"n_periodic\": {},\n  \"n_boundary\": {},\n  \
         \"n_transitions\": {},\n  \
         \"periodic_z\": [\n    {}\n  ],\n  \"boundary\": [\n    {}\n  ],\n  \
         \"transitions_z\": [\n    {}\n  ]\n}}\n",
        fp2_hex(&r.comp_z),
        periodic_z.len(),
        boundary.len(),
        transitions_z.len(),
        periodic_z.join(",\n    "),
        boundary.join(",\n    "),
        transitions_z.join(",\n    "),
    );
    let comp_out = out.replace("replay-fixture", "composition-fixture");
    std::fs::write(&comp_out, &comp).expect("write composition fixture");
    println!("wrote {comp_out}");
    println!(
        "z={} comp_z={} indices[0..4]={:?}",
        fp2_hex(&r.z),
        fp2_hex(&r.comp_z),
        &r.indices[..4.min(r.indices.len())]
    );
}
