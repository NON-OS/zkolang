/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use super::mds::POOL_LOG_ROUNDS;
use super::render::render;
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;
use std::fs;
use std::path::PathBuf;

pub(super) fn circuit_path() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.push("circuits/shield/note_commit.zkl");
    p
}

#[test]
#[ignore]
fn regenerate_note_commit_circuit() {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let src = render(&h, 1usize << POOL_LOG_ROUNDS);
    fs::write(circuit_path(), src).expect("write circuit");
}
