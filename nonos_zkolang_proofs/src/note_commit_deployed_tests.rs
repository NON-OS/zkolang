/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use crate::note_commit_tests::in_language;
use nonos_stark::air::RATE;

/* The deployed digest, not a locally built one.
 *
 * Every test of `note_commit_tests` compares the circuit against a `Poseidon`
 * this crate constructs, which any self consistent pair of round counts
 * satisfies. That is how the circuit spent time on a four round hash while
 * `ShieldedPool` commits under thirty two: the comparison was internally honest
 * and pointed at the wrong target, so a note this circuit produced would hash
 * to something `deposit` never computes and would sit in no tree.
 *
 * This value is `commit_note([1..11])` as the STARKs repository pins it, in
 * `stark_proofs/src/shield/test/pool_hash.rs`,
 * `the_pool_hash_is_frozen_to_the_deployed_digest`, the digest
 * `PoseidonGoldilocks.commitNote` is gated against. So the chain runs from the
 * circuit through nonos-stark to the deployed hasher. If it fails, the circuit
 * is committing notes the pool cannot recognise. Regenerate the circuit rather
 * than editing the expectation. */
#[test]
fn matches_the_deployed_commitment() {
    let got = in_language(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    let deployed: [u64; RATE] = [
        3128788525238172940,
        977760251882506394,
        7248597715382424175,
        11116448888154628040,
    ];
    assert_eq!(
        got, deployed,
        "the circuit does not compute the deployed note commitment"
    );
}
