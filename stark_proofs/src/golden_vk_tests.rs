// NONOS Operating System (AGPL-3.0-or-later)
//! The golden verifier key: the recursion attests a specific zkolang program, so
//! the inner it proves must be the one the registry names. This checks that the
//! periodic root the recursion's inner commits equals the root `verifier_key`
//! binds, and that the vkey chain reproduces — so an on-chain verifier matching
//! the recursion's exposed inner against a registered `verifier_key(program, 3)`
//! is matching one object, not a coincidence. No prove: roots only, so it is fast.

use crate::crypto::stark::air::periodic_root;
use crate::recursion_assembly::inner::{hasher, step_air};
use nonos_zkolang::{
    compile_source, periodic_root as zk_periodic_root, registration_root, verifier_key,
    REGISTRATION_RATE,
};

const PROGRAM: &str = "input x; let y = x * x; output y;";

#[test]
fn recursion_inner_root_matches_the_verifier_key() {
    let program = compile_source(PROGRAM).expect("compile");

    // The root the vkey binds, built at the driver's log_t.
    let vk_root = zk_periodic_root(&program, REGISTRATION_RATE).expect("vkey periodic root");

    // The root the recursion's inner actually commits. If step_air sizes the inner
    // at a different log_t than the driver, this diverges — the descriptor skew.
    let inner = step_air(&hasher());
    let inner_root = periodic_root(&inner.air, REGISTRATION_RATE);

    assert_eq!(
        inner_root, vk_root,
        "the recursion inner's periodic root differs from verifier_key's — log_t/descriptor skew"
    );
}

#[test]
fn verifier_key_chain_reproduces() {
    let program = compile_source(PROGRAM).expect("compile");
    let root = zk_periodic_root(&program, REGISTRATION_RATE).expect("root");
    // registration_root is periodic_root at the registration rate: one object.
    assert_eq!(registration_root(&program).expect("registration root"), root);
    // The key is deterministic from the program + rate.
    let vk = verifier_key(&program, REGISTRATION_RATE).expect("vk");
    assert_eq!(vk, verifier_key(&program, REGISTRATION_RATE).expect("vk again"));
}
