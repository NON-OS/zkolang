/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use super::mds::{mds, POOL_LOG_ROUNDS};
use super::write::circuit_path;
use nonos_stark::air::{Poseidon, RATE, WIDTH};
use nonos_stark::field::Fp;
use std::fs;

/// The generator rebuilds the permutation from the public API. If that is wrong
/// every table it writes is confidently wrong, so it is checked against the hash.
#[test]
fn the_reconstructed_permutation_matches_the_hash() {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let rounds = 1usize << POOL_LOG_ROUNDS;
    let m = mds(&h);
    let mut state = [Fp::ZERO; WIDTH];
    for (i, v) in state.iter_mut().enumerate() {
        *v = Fp::from_u64(i as u64 + 1);
    }
    let mut mine = state;
    for r in 0..rounds {
        let rc = h.round_constant(r);
        let sb: Vec<Fp> = mine.iter().map(|v| v.pow(7)).collect();
        let mut out = [Fp::ZERO; WIDTH];
        for (j, o) in out.iter_mut().enumerate() {
            let mut acc = rc[j];
            for (i, c) in m[j].iter().enumerate() {
                acc = acc + *c * sb[i];
            }
            *o = acc;
        }
        mine = out;
    }
    assert_eq!(mine, h.permute(state));
}

/// A circuit whose round count drifts from the pool's commits notes the pool
/// cannot recognise.
#[test]
fn the_committed_circuit_matches_the_pool_round_count() {
    let src = fs::read_to_string(circuit_path()).expect("read circuit");
    let rounds = 1usize << POOL_LOG_ROUNDS;
    assert!(src.contains(&format!("fn mix{}(s)", rounds - 1)));
    assert!(!src.contains(&format!("fn mix{rounds}(s)")));
}

#[test]
fn the_commitment_circuit_fits_its_op_budget() {
    let ops = nonos_zkolang::compile_source(&fs::read_to_string(circuit_path()).unwrap())
        .expect("compile");
    assert!(
        ops.len() < 30_000,
        "note_commit compiles to {} ops",
        ops.len()
    );
}
