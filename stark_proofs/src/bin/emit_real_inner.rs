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

use stark_proofs::crypto::stark::air::{stark_verify_poseidon_ext_pub, Air};
use stark_proofs::recursion_assembly::inner::{hasher, shield_join_split, EXTRA, GRIND, NQ};
use std::time::Instant;

fn main() {
    let h = hasher();

    let t0 = Instant::now();
    let inner = shield_join_split(&h);
    let built = t0.elapsed();

    println!(
        "inner     log_trace_len={} t={} degree={} periodic={} publics={}",
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
