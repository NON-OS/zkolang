/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The larger example programs, each proven twice: once on an honest witness that must
//! verify, and once on a witness that breaks the statement and so must have no proof. The
//! public inputs a hash-based program needs are computed by the same program, run in a
//! variant that outputs the value it would otherwise check, so the test never keeps a
//! second copy of the circuit's arithmetic.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_witness};

fn program(name: &str) -> String {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.pop();
    let src = fs::read_to_string(root.join("examples").join(name)).expect("read example");
    let mut resolve = |inc: &str| {
        for dir in ["stdlib", "examples"] {
            if let Ok(s) = fs::read_to_string(root.join(dir).join(inc)) {
                return Some(s);
            }
        }
        None
    };
    expand_includes(&src, &mut resolve).expect("expand includes")
}

fn outputs(src: &str, public: &[u64], witness: &[u64]) -> Vec<u64> {
    let report = prove_source_with_witness(src, public, witness).expect("run");
    assert!(report.verified);
    report.outputs
}

fn rejected(src: &str, public: &[u64], witness: &[u64]) -> bool {
    match prove_source_with_witness(src, public, witness) {
        Err(_) => true,
        Ok(report) => !report.verified,
    }
}

#[test]
fn allowlist_admits_a_member_and_refuses_a_stranger() {
    let src = program("allowlist.zkl");
    let allow = [10u64, 20, 30, 40, 50];
    assert!(
        prove_source_with_witness(&src, &allow, &[30])
            .expect("prove")
            .verified,
        "a member was refused"
    );
    assert!(rejected(&src, &allow, &[99]), "a stranger was admitted");
}

#[test]
fn vote_tally_counts_bits_and_rejects_a_stuffed_ballot() {
    let src = program("vote_tally.zkl");
    // Four yes votes among seven ballots.
    let ballots = [1u64, 0, 1, 1, 0, 0, 1];
    assert!(
        prove_source_with_witness(&src, &[4], &ballots)
            .expect("prove")
            .verified,
        "an honest tally was rejected"
    );
    // A ballot worth two is not a bit, so it has no proof even if the sum is claimed right.
    let stuffed = [2u64, 0, 1, 1, 0, 0, 1];
    assert!(rejected(&src, &[5], &stuffed), "a stuffed ballot counted");
}

#[test]
fn poly_open_is_consistent_with_its_commitment() {
    let src = program("poly_open.zkl");
    let coeffs = [3u64, 5, 7, 11];
    let x = 2u64;
    // Derive the commitment and the evaluation from the program itself.
    let computer = src
        .replace("input digest;\n", "")
        .replace("input y;\n", "")
        .replace(
            "assert commit4(c0, c1, c2, c3) == digest;",
            "output commit4(c0, c1, c2, c3);",
        )
        .replace(
            "assert horner4(c0, c1, c2, c3, x) == y;",
            "output horner4(c0, c1, c2, c3, x);",
        );
    let vals = outputs(&computer, &[x], &coeffs);
    let (digest, y) = (vals[0], vals[1]);
    assert_eq!(y, 129, "3 + 5*2 + 7*4 + 11*8");
    // The opening at (x, y) is consistent with the commitment.
    assert!(
        prove_source_with_witness(&src, &[digest, x, y], &coeffs)
            .expect("prove")
            .verified,
        "an honest opening was rejected"
    );
    // A different evaluation at the same point does not open the committed polynomial.
    assert!(
        rejected(&src, &[digest, x, y + 1], &coeffs),
        "a false opening verified"
    );
}

#[test]
fn id_proof_binds_the_key_to_the_challenge() {
    let src = program("id_proof.zkl");
    let sk = 123_456u64;
    let challenge = 42u64;
    let computer = src
        .replace("input identity;\n", "")
        .replace("input response;\n", "")
        .replace("assert permute(sk) == identity;", "output permute(sk);")
        .replace(
            "assert permute(sk + challenge) == response;",
            "output permute(sk + challenge);",
        );
    let vals = outputs(&computer, &[challenge], &[sk]);
    let (identity, response) = (vals[0], vals[1]);
    assert!(
        prove_source_with_witness(&src, &[identity, challenge, response], &[sk])
            .expect("prove")
            .verified,
        "an honest identification was rejected"
    );
    // A response for a different challenge cannot be replayed against this one.
    assert!(
        rejected(&src, &[identity, challenge, response + 1], &[sk]),
        "a forged response verified"
    );
}

#[test]
fn rollup_update_changes_one_leaf_between_roots() {
    let src = program("rollup_update.zkl");
    // old_balance, delta, owner, sib0, sib1, dir0, dir1.
    let witness = [100u64, 50, 7, 11, 13, 1, 0];
    let computer = src
        .replace("input old_root;\n", "")
        .replace("input new_root;\n", "")
        .replace(
            "assert climb(account_leaf(owner, old_balance), sib0, sib1, dir0, dir1) == old_root;",
            "output climb(account_leaf(owner, old_balance), sib0, sib1, dir0, dir1);",
        )
        .replace(
            "assert climb(account_leaf(owner, new_balance), sib0, sib1, dir0, dir1) == new_root;",
            "output climb(account_leaf(owner, new_balance), sib0, sib1, dir0, dir1);",
        );
    let vals = outputs(&computer, &[], &witness);
    let (old_root, new_root) = (vals[0], vals[1]);
    assert_ne!(old_root, new_root, "the update did not change the root");
    assert!(
        prove_source_with_witness(&src, &[old_root, new_root], &witness)
            .expect("prove")
            .verified,
        "an honest transition was rejected"
    );
    // A new root that is not the tree with this leaf updated has no proof.
    assert!(
        rejected(&src, &[old_root, new_root + 1], &witness),
        "a wrong new root verified"
    );
}
