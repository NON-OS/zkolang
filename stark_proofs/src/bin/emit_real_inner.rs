// NONOS Operating System (AGPL-3.0-or-later)
//! Prove a real transfer the way the recursion needs it proven, and time it.
//!
//! The recursion has always verified a stand-in: two toy regions with synthetic
//! publics, sized so the assembly could be built at all. This proves the
//! deployed join-split instead, with the Poseidon transcript the recursion
//! replays in circuit and the intent absorbed as its publics.
//!
//! It proves at one of two soundness points. The default is settlement: 32
//! queries against a rate-1/16 domain, the point the registered keys hold, tuned
//! for a small proof that lands on chain once per batch. `transfer` proves the
//! same circuit at the sender's point: more queries against a rate-1/4 domain,
//! the same 128 bits over a quarter of the domain, for the proof a wallet makes
//! and the recursion verifies off chain. Run both and the cost of a transaction
//! and the cost of a settlement are two measured numbers, not one standing in
//! for both.
//!
//! With `--hidden` it proves at the settlement point with the trace blinded, the
//! private-transfer form, from a fixed seed so the emit is a reproducible known
//! answer; production draws a fresh seed per proof from the capsule CSPRNG.
//!
//! The proof is preprocessed: its transcript absorbs the periodic claims and its
//! consistency queries open against a baked periodic root, so it is checked by
//! the preprocessed verifier against that root. The plain verifier replays a
//! transcript that never saw the claims and rejects such a proof by construction,
//! which is not a verdict on the proof.

use stark_proofs::crypto::stark::air::{
    seed_from_entropy, stark_verify_poseidon_pre_pub, Air, StarkProofExtPPre,
};
use stark_proofs::recursion_assembly::inner::{
    hasher, shield_join_split_at, shield_join_split_hidden,
};
use stark_proofs::shield_params::{deployment, transfer};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let hidden = args.iter().any(|a| a == "--hidden");
    /*
     * The hidden emit is a settlement-point known answer, so `transfer` only
     * selects the point when the trace is left plain.
     */
    let at_transfer = !hidden && args.iter().any(|a| a == "transfer");

    let (nq, grind, extra, point) = if at_transfer {
        (
            transfer::N_QUERIES,
            transfer::GRIND_BITS,
            transfer::EXTRA_BLOWUP_BITS,
            "transfer",
        )
    } else {
        (
            deployment::N_QUERIES,
            deployment::GRIND_BITS,
            deployment::EXTRA_BLOWUP_BITS,
            "settlement",
        )
    };
    let h = hasher();

    let t0 = Instant::now();
    let inner = if hidden {
        /*
         * A fixed, documented seed so the hidden emit is reproducible for a KAT.
         * The bytes are arbitrary and public; the point is that production
         * replaces this with a fresh CSPRNG draw, while the KAT pins one answer.
         */
        let mut bytes = [0u8; 96];
        for (k, b) in bytes.iter_mut().enumerate() {
            *b = (11 * k + 7) as u8;
        }
        let seed = seed_from_entropy(&bytes).expect("fixed seed buffer is long enough");
        shield_join_split_hidden(&h, &seed)
    } else {
        shield_join_split_at(&h, nq, grind, extra)
    };
    let built = t0.elapsed();

    println!(
        "inner     point={point} log_trace_len={} t={} degree={} periodic={} publics={} hidden={hidden}",
        inner.air.log_trace_len(),
        inner.t,
        inner.air.constraint_degree(),
        inner.air.periodic_columns().len(),
        inner.publics.len()
    );
    println!("proved in {built:?}  ({nq} queries, grind {grind}, extra {extra})");
    println!("proof     1 wide trace root, {nq} queries committed");

    let sidecar = inner
        .sidecar
        .as_ref()
        .expect("the deployed inner carries its periodic sidecar");
    let pre = StarkProofExtPPre {
        proof: inner.proof.clone(),
        periodic_z: sidecar.periodic_z.clone(),
        openings: sidecar.openings.clone(),
    };

    let t1 = Instant::now();
    let ok = stark_verify_poseidon_pre_pub(
        &inner.air,
        &pre,
        nq,
        grind,
        extra,
        &h,
        &inner.publics,
        &sidecar.root,
    );
    println!("verified in {:?}: {ok}", t1.elapsed());

    if !ok {
        eprintln!("the real inner proof did not verify");
        std::process::exit(1);
    }
}
