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
    let pre = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[], &[]);
    assert!(
        stark_verify_poseidon_pre_pub(&air, &pre, NQ, GRIND, EXTRA, &h, &[], &root),
        "an honest sidecar proof must verify against the registered root"
    );
}

#[test]
fn a_wrong_periodic_root_rejects() {
    let (air, witness, h, mut root) = setup();
    let pre = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[], &[]);
    root[0] = root[0] + Fp::ONE;
    assert!(
        !stark_verify_poseidon_pre_pub(&air, &pre, NQ, GRIND, EXTRA, &h, &[], &root),
        "a proof must not verify against a root it was not committed under"
    );
}

#[test]
fn a_tampered_periodic_claim_rejects() {
    let (air, witness, h, root) = setup();
    let mut pre = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[], &[]);
    pre.periodic_z[0] = pre.periodic_z[0] + crate::crypto::stark::field::Fp2::ONE;
    assert!(
        !stark_verify_poseidon_pre_pub(&air, &pre, NQ, GRIND, EXTRA, &h, &[], &root),
        "a bent periodic claim must fail the composition or the quotient"
    );
}

#[test]
fn a_tampered_periodic_opening_rejects() {
    let (air, witness, h, root) = setup();
    let mut pre = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[], &[]);
    pre.openings[0].row[0] = pre.openings[0].row[0] + Fp::ONE;
    assert!(
        !stark_verify_poseidon_pre_pub(&air, &pre, NQ, GRIND, EXTRA, &h, &[], &root),
        "a bent opened row must fail its path to the baked root"
    );
}

/// The deployed transfer runs on this preprocessed prover, and the private-transfer
/// cutover has it pass a per-proof blinding. This proves that path sound before it is
/// flipped on: a blinded sidecar proof still verifies against the baked root, and its
/// out-of-domain frame moves, so the blinding reaches the openings the FRI queries
/// read. The blind degree is capped at what the fixture's composition bound admits;
/// the deployed circuit sizes far above the query count and so carries a full-strength
/// blind, while this small fixture only shows the preprocessed path accepts one.
#[test]
fn a_blinded_sidecar_proof_verifies_and_moves_the_frame() {
    use crate::crypto::stark::air::{blinding_poly, Air};
    let (air, witness, h, root) = setup();

    let plain = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[], &[]);
    assert!(
        stark_verify_poseidon_pre_pub(&air, &plain, NQ, GRIND, EXTRA, &h, &[], &root),
        "the plain sidecar proof must verify"
    );

    /*
     * The largest blind the fixture's composition bound admits, capped at the query
     * count. Each column rises to degree t + b, so the blinded composition must stay
     * under B = (degree * t).next_power_of_two(); this is the same bound the driver
     * enforces, computed from the fixture's own AIR so the test cannot outrun it.
     */
    let t = 1usize << air.log_trace_len();
    let degree = air.constraint_degree().max(1);
    let bound = (degree * t).next_power_of_two();
    let fit = (bound + t).saturating_sub(degree * t + air.window_size()) / degree;
    let deg = (NQ + air.window_size()).min(fit);
    assert!(deg >= 1, "the fixture must admit at least a degree-one blind");

    let seed = [Fp::from_u64(5), Fp::from_u64(6), Fp::from_u64(7), Fp::from_u64(8)];
    let blind: alloc::vec::Vec<alloc::vec::Vec<Fp>> =
        (0..air.trace_width()).map(|c| blinding_poly(&h, &seed, c, deg)).collect();

    let zk = stark_prove_poseidon_pre_pub(&air, &witness, NQ, GRIND, EXTRA, &h, &[], &blind);
    assert!(
        stark_verify_poseidon_pre_pub(&air, &zk, NQ, GRIND, EXTRA, &h, &[], &root),
        "a blinded sidecar proof must still verify against the baked root"
    );
    assert_ne!(
        plain.proof.ood_frame, zk.proof.ood_frame,
        "blinding must move the out-of-domain frame the queries read"
    );
}

/// The private-transfer cutover has the deployed prover pass a blinding of the
/// exposed point count, the query rows plus the out-of-domain frame. This proves the
/// deployed circuit is large enough to carry one: its composition bound admits a
/// blind of degree at least `N_QUERIES + window`, so the cutover cannot silently
/// under-blind the real transfer. The margin is read from the deployed AIR itself,
/// not a hand bound, so a circuit change that shrank it below the exposed count
/// fails here rather than in production.
#[test]
fn the_deployed_transfer_admits_a_full_strength_blind() {
    use crate::crypto::stark::air::Air;
    let js = crate::shield::test::scenario::balanced_deployed(crate::shield::key::Break::None);
    let air = &js.wired;
    let t = 1usize << air.log_trace_len();
    let degree = air.constraint_degree().max(1);
    let bound = (degree * t).next_power_of_two();
    let fit = (bound + t).saturating_sub(degree * t + air.window_size()) / degree;
    let need = crate::shield_params::deployment::N_QUERIES + air.window_size();
    assert!(
        fit >= need,
        "the deployed transfer must admit an exposed-count blinding: room for {fit}, need {need}"
    );
}
