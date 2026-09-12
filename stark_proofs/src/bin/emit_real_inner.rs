// NONOS Operating System (AGPL-3.0-or-later)
//! Prove a real transfer the way the recursion needs it proven.
//!
//! The recursion has always verified a stand-in: two toy regions with synthetic
//! publics, sized so the assembly could be built at all. This proves the
//! deployed join-split instead, with the Poseidon transcript the recursion
//! replays in circuit and the intent absorbed as its publics.
//!
//! Until this holds there is no point wiring the real circuit into the
//! recursion, because the inner proof it would verify does not exist.
//!
//! With `--hidden` it proves the same transfer with the trace blinded, the
//! private-transfer form. The blind comes from a fixed seed here so the emit is a
//! reproducible known answer for the prover; production draws a fresh seed per
//! proof from the capsule CSPRNG. The proof verifies under the same plain verifier
//! either way, since the blinding is invisible to it.

use stark_proofs::crypto::stark::air::{seed_from_entropy, stark_verify_poseidon_ext_pub, Air};
use stark_proofs::recursion_assembly::inner::{
    hasher, shield_join_split, shield_join_split_hidden, EXTRA, GRIND, NQ,
};
use std::time::Instant;

fn main() {
    let hidden = std::env::args().any(|a| a == "--hidden");
    let h = hasher();

    let t0 = Instant::now();
    let inner = if hidden {
        // A fixed, documented seed so the hidden emit is reproducible for a KAT.
        // The bytes are arbitrary and public; the point is that production replaces
        // this with a fresh CSPRNG draw, while the KAT pins one seed's answer.
        let mut bytes = [0u8; 96];
        for (k, b) in bytes.iter_mut().enumerate() {
            *b = (11 * k + 7) as u8;
        }
        let seed = seed_from_entropy(&bytes).expect("fixed seed buffer is long enough");
        shield_join_split_hidden(&h, &seed)
    } else {
        shield_join_split(&h)
    };
    let built = t0.elapsed();

    println!(
        "inner     log_trace_len={} t={} degree={} periodic={} publics={} hidden={hidden}",
        inner.air.log_trace_len(),
        inner.t,
        inner.air.constraint_degree(),
        inner.air.periodic_columns().len(),
        inner.publics.len()
    );
    println!("proved in {built:?}  ({NQ} queries, grind {GRIND}, extra {EXTRA})");

    println!("proof     1 wide trace root, {NQ} queries committed");

    let t1 = Instant::now();
    let ok = stark_verify_poseidon_ext_pub(
        &inner.air,
        &inner.proof,
        NQ,
        GRIND,
        EXTRA,
        &h,
        &inner.publics,
    );
    println!("verified in {:?}: {ok}", t1.elapsed());

    if !ok {
        eprintln!("the real inner proof did not verify");
        std::process::exit(1);
    }
}
