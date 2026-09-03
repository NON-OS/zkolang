// NONOS Operating System (AGPL-3.0-or-later)
//! The poseidon preprocessed path, gated the only way that counts: an honest
//! sidecar proof verifies against the baked root, and every part of the
//! sidecar rejects when bent. The fixture inner keeps the gate fast; the
//! protocol is size-blind.

use crate::crypto::stark::air::{
    periodic_root_poseidon, stark_prove_poseidon_pre_pub, stark_verify_poseidon_pre_pub,
};
use crate::crypto::stark::field::Fp;
use crate::recursion_assembly::inner;

const NQ: usize = 8;
const GRIND: u32 = 4;
const EXTRA: u32 = 1;

fn setup() -> (
    crate::crypto::stark::air::WiredExt,
    alloc::vec::Vec<Fp>,
    crate::crypto::stark::air::Poseidon,
    [Fp; 4],
) {
    let h = inner::hasher();
    let (air, witness, _publics) = inner::join_split_fixture();
    let root = periodic_root_poseidon(&air, EXTRA, &h);
    (air, witness, h, root)
}

#[test]
fn the_sidecar_proof_verifies_against_the_baked_root() {
    let (air, witness, h, root) = setup();
    let pre = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[]);
    assert!(
        stark_verify_poseidon_pre_pub(&air, &pre, NQ, GRIND, EXTRA, &h, &[], &root),
        "an honest sidecar proof must verify against the registered root"
    );
}

#[test]
fn a_wrong_periodic_root_rejects() {
    let (air, witness, h, mut root) = setup();
    let pre = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[]);
    root[0] = root[0] + Fp::ONE;
    assert!(
        !stark_verify_poseidon_pre_pub(&air, &pre, NQ, GRIND, EXTRA, &h, &[], &root),
        "a proof must not verify against a root it was not committed under"
    );
}

#[test]
fn a_tampered_periodic_claim_rejects() {
    let (air, witness, h, root) = setup();
    let mut pre = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[]);
    pre.periodic_z[0] = pre.periodic_z[0] + crate::crypto::stark::field::Fp2::ONE;
    assert!(
        !stark_verify_poseidon_pre_pub(&air, &pre, NQ, GRIND, EXTRA, &h, &[], &root),
        "a bent periodic claim must fail the composition or the quotient"
    );
}

#[test]
fn a_tampered_periodic_opening_rejects() {
    let (air, witness, h, root) = setup();
    let mut pre = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[]);
    pre.openings[0].row[0] = pre.openings[0].row[0] + Fp::ONE;
    assert!(
        !stark_verify_poseidon_pre_pub(&air, &pre, NQ, GRIND, EXTRA, &h, &[], &root),
        "a bent opened row must fail its path to the baked root"
    );
}
