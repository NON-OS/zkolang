// NONOS Operating System (AGPL-3.0-or-later)
//! The first spend: two real notes in a deployed pool, unshielded in full.
//!
//! Everything here is a value read off Sepolia or handed over with it, not a
//! fixture. The pool is
//! `0xa760D749adfe15BafFfeC8E7301CEA89B150c046`, the two spendable notes sit
//! at leaves 1 and 2, and leaf 0 is an earlier note nobody holds the secret
//! for. The commitments were read from the pool's own `NoteCommitted` logs
//! rather than copied from the handoff, so the leaves proved here are the
//! leaves the contract stored.
//!
//! Kept in the crate rather than in the test tree because the emitter that
//! produces the proof needs it, and because a spend against live state is
//! worth being able to rebuild exactly a month from now.

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::join::Witnessed;
use crate::shield::join::{
    address_from_u64, join_split_published, join_split_with_paths, AssocAnchor, JoinSplit, Settle,
    Spend,
};
use crate::shield::key::Break;
use crate::shield::member::PoolTree;
use crate::shield::note::{Note, POOL_LOG_ROUNDS};
use alloc::vec::Vec;

/// The pool's tree depth, which the contract fixes at deployment.
pub const DEPTH: usize = 32;

/// Leaf 0: an earlier deposit whose secret nobody has. Present only because
/// it is leaf 1's sibling, so no opening can be built without it.
pub const LEAF0_CM: &str = "0f14542ea9ec2683cac6fb8e8167203c8c7eed6759da835fd29c233259e8ba62";

/// Note A, leaf 1.
pub const A_SK: &str = "9c4b73105bd8c464250d949588b61807aee31121ea9d916ff6bf6105dab3ff7e";
pub const A_BLINDING: &str = "feb675b9c81199eac575cd15116a9215f5289e96d9706e1afffa4e4fcd9affe1";
pub const A_CM: &str = "59b6835c74c863e7b9defa6bc7c34e1ad9ee1c8519a6fe0f301a2502cd927fd8";

/// Note B, leaf 2.
pub const B_SK: &str = "0730018f3ed1e773658f0e36f8289c82d04608ca0589548e4a55cef505ad496e";
pub const B_BLINDING: &str = "8c217b2d9fa1e72cddedcef23597d3ce913d14c7e34977e49fa9646fadeffd28";
pub const B_CM: &str = "1edff88b113db999adadd2406035b75c13c3eb69ee244dc4f56ecaf36e544897";

/// Both notes hold the same post-fee value; the deposits were equal.
pub const NOTE_VALUE: u64 = 1_995_000_000_000_000;

/// The root after leaf 2, which holds both notes and which the pool reports
/// as known.
pub const ROOT: &str = "e083f588523bb322c7d4077ecf9d36f1628a2655c22c86136c73b6b3da159a3f";

/// A `bytes32` as the circuit's four limbs: big endian over
/// `[limb3, limb2, limb1, limb0]`, so limb 0 is the last eight bytes. Read
/// the other way round every value is still a valid field element and every
/// note is unspendable, which is the failure worth naming.
pub fn limbs(hex: &str) -> [Fp; RATE] {
    let b: Vec<u8> = (0..32)
        .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
        .collect();
    let mut out = [Fp::ZERO; RATE];
    for (j, slot) in out.iter_mut().enumerate() {
        let hi = 24 - 8 * j;
        let mut w = [0u8; 8];
        w.copy_from_slice(&b[hi..hi + 8]);
        *slot = Fp::from_u64(u64::from_be_bytes(w));
    }
    out
}

pub fn hex_of(d: [Fp; RATE]) -> alloc::string::String {
    let mut s = alloc::string::String::new();
    for j in (0..RATE).rev() {
        s.push_str(&alloc::format!("{:016x}", d[j].to_u64()));
    }
    s
}

pub fn hasher() -> Poseidon {
    Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
}

fn u64s(d: [Fp; RATE]) -> [u64; 4] {
    [d[0].to_u64(), d[1].to_u64(), d[2].to_u64(), d[3].to_u64()]
}

/// The note a secret and a blinding describe, with the spend key derived
/// rather than supplied: a note whose committed key is not the one its
/// secret derives is a note nobody can spend, and deriving it here means
/// that cannot be got wrong by transcription.
fn note_of(sk_hex: &str, blinding_hex: &str, value: u64) -> Note {
    let k = crate::shield::key::derive(&hasher(), limbs(sk_hex));
    Note {
        value,
        asset_id: 0,
        spend_pk: u64s(k.spend_pk),
        blinding: u64s(limbs(blinding_hex)),
    }
}

/// The association set the registry published, as the contracts lane
/// registered it. The two spent notes must open under this root, or
/// `settleBatch` refuses the intent before it examines the proof.
pub const ASSOC_ROOT: &str = "af3f40b52f9cc92a85433b5327fc4b2f3fa13c2885765ae2c540fe5cd1c16cf8";

/// The first spend, proved the way production proves one: both trees
/// published, neither planted.
///
/// `unshield` below still uses the planted association set, because the
/// registry's openings are not something this crate can invent and the
/// contracts lane has to supply them. This is the entry that takes them.
pub fn unshield_published(
    assoc_openings: [&crate::shield::join::Witnessed; 2],
    assoc_root: [Fp; RATE],
    recipient: u64,
    clearing_price: u64,
) -> JoinSplit {
    let a = note_of(A_SK, A_BLINDING, NOTE_VALUE);
    let b = note_of(B_SK, B_BLINDING, NOTE_VALUE);
    let zero_a = Note {
        value: 0,
        asset_id: 0,
        spend_pk: [1, 2, 3, 4],
        blinding: [5, 6, 7, 8],
    };
    let zero_b = Note {
        value: 0,
        asset_id: 0,
        spend_pk: [9, 10, 11, 12],
        blinding: [13, 14, 15, 16],
    };
    let t = tree();
    let open_a = crate::shield::join::Witnessed {
        leaf_index: 1,
        siblings: t.path(1).0,
    };
    let open_b = crate::shield::join::Witnessed {
        leaf_index: 2,
        siblings: t.path(2).0,
    };

    join_split_published(
        DEPTH,
        [
            Spend {
                note: &a,
                sk: limbs(A_SK),
            },
            Spend {
                note: &b,
                sk: limbs(B_SK),
            },
        ],
        [&open_a, &open_b],
        limbs(ROOT),
        AssocAnchor {
            openings: assoc_openings,
            root: assoc_root,
        },
        [&zero_a, &zero_b],
        NOTE_VALUE * 2,
        0,
        Break::None,
        Settle {
            clearing_price,
            recipient: address_from_u64(recipient),
        },
        None,
    )
}

/// The pool's tree as the chain holds it: three leaves, in order.
pub fn tree() -> PoolTree {
    let mut t = PoolTree::with_depth(hasher(), DEPTH);
    for cm in [LEAF0_CM, A_CM, B_CM] {
        t.insert(limbs(cm));
    }
    t
}

/// The first spend: both notes in, nothing kept back, the whole value out to
/// `recipient` as a public amount.
///
/// The outputs are notes worth zero. They are not absent: the circuit has
/// two output legs and commits both, so the pool will insert two real
/// commitments as leaves. Their blindings are fixed rather than random
/// because nothing can ever be spent from them and a reproducible spend is
/// worth more here than an unpredictable dead leaf.
pub fn unshield(recipient: u64, clearing_price: u64) -> JoinSplit {
    let a = note_of(A_SK, A_BLINDING, NOTE_VALUE);
    let b = note_of(B_SK, B_BLINDING, NOTE_VALUE);
    let zero_a = Note {
        value: 0,
        asset_id: 0,
        spend_pk: [1, 2, 3, 4],
        blinding: [5, 6, 7, 8],
    };
    let zero_b = Note {
        value: 0,
        asset_id: 0,
        spend_pk: [9, 10, 11, 12],
        blinding: [13, 14, 15, 16],
    };

    let t = tree();
    let (a_sibs, _) = t.path(1);
    let (b_sibs, _) = t.path(2);
    let open_a = Witnessed {
        leaf_index: 1,
        siblings: a_sibs,
    };
    let open_b = Witnessed {
        leaf_index: 2,
        siblings: b_sibs,
    };

    join_split_with_paths(
        DEPTH,
        [
            Spend {
                note: &a,
                sk: limbs(A_SK),
            },
            Spend {
                note: &b,
                sk: limbs(B_SK),
            },
        ],
        [&open_a, &open_b],
        limbs(ROOT),
        [&zero_a, &zero_b],
        NOTE_VALUE * 2,
        0,
        Break::None,
        Settle {
            clearing_price,
            recipient: address_from_u64(recipient),
        },
        None,
    )
}

/// A note's commitment, the way the pool computes it: the eleven limbs in
/// two compressions and then one more. Used to plant a synthetic tree whose
/// leaves are real commitments rather than arbitrary digests, so the
/// openings a spend proves against are the openings a pool would give.
fn commitment_of(note: &Note) -> [Fp; RATE] {
    let h = hasher();
    let q = crate::shield::note::quads(&note.limbs());
    let d0 = h.compress(&q[0], &q[1]);
    let d1 = h.compress(&q[2], &q[3]);
    h.compress(&d0, &d1)
}

/// A spend of two notes sitting at chosen leaf positions in a tree of
/// `leaves` notes, for counting how many distinct circuits a pool needs.
///
/// Deployability does not turn on why the wiring varies, only on how many
/// ways it can vary. If the count is small and bounded, a pool launches one
/// verifier per variant and the settler declares which at open; a wrong
/// declaration is self detecting, because the periodic root will not match
/// and the walk fails. If the count grows with the leaf index, no
/// deployment exists and the circuit has to change.
pub fn unshield_at(ia: usize, ib: usize, leaves: usize, value: u64) -> JoinSplit {
    assert!(
        ia < leaves && ib < leaves && ia != ib,
        "two distinct positions inside the tree"
    );
    let h = hasher();
    let sk_a = limbs(A_SK);
    let sk_b = limbs(B_SK);
    let a = Note {
        value,
        asset_id: 0,
        spend_pk: u64s(crate::shield::key::derive(&h, sk_a).spend_pk),
        blinding: [11, 22, 33, 44],
    };
    let b = Note {
        value,
        asset_id: 0,
        spend_pk: u64s(crate::shield::key::derive(&h, sk_b).spend_pk),
        blinding: [55, 66, 77, 88],
    };
    let zero_a = Note {
        value: 0,
        asset_id: 0,
        spend_pk: [21, 22, 23, 24],
        blinding: [25, 26, 27, 28],
    };
    let zero_b = Note {
        value: 0,
        asset_id: 0,
        spend_pk: [29, 30, 31, 32],
        blinding: [33, 34, 35, 36],
    };

    let mut t = PoolTree::with_depth(h.clone(), DEPTH);
    for i in 0..leaves {
        if i == ia {
            t.insert(commitment_of(&a));
        } else if i == ib {
            t.insert(commitment_of(&b));
        } else {
            t.insert([Fp::from_u64(1000 + i as u64); RATE]);
        }
    }
    let open_a = Witnessed {
        leaf_index: ia,
        siblings: t.path(ia).0,
    };
    let open_b = Witnessed {
        leaf_index: ib,
        siblings: t.path(ib).0,
    };

    join_split_with_paths(
        DEPTH,
        [Spend { note: &a, sk: sk_a }, Spend { note: &b, sk: sk_b }],
        [&open_a, &open_b],
        t.root(),
        [&zero_a, &zero_b],
        value * 2,
        0,
        Break::None,
        Settle {
            clearing_price: 1_000_000,
            recipient: address_from_u64(BURNER_ADDR),
        },
        None,
    )
}

/// The burner the contracts lane named as the payout address.
pub const BURNER_ADDR: u64 = 0x7408_ae4c;

/// A second spend that differs in everything a production spend varies: a
/// different pool tree, different leaf indices, different siblings, different
/// note values, a different recipient and a different published root.
///
/// The contracts lane asked for exactly this and was right to: two spends
/// against the *same* root would share a periodic root even on a circuit
/// that baked the tree in, so that test would pass on a broken circuit. This
/// one cannot. If these two spends share a periodic set, one deployed
/// verifier serves every spend against the pool. If they do not, every spend
/// needs its own verifier and the design is undeployable.
pub fn unshield_variant() -> JoinSplit {
    let h = hasher();
    /*
     * Six leaves rather than three, the spent pair sitting at 3 and 4 rather
     * than 1 and 2, so the openings differ in length of shared prefix, in
     * direction bits and in every sibling.
     */
    let sk_a = limbs("11112222333344445555666677778888999900001111222233334444aaaa0001");
    let sk_b = limbs("22223333444455556666777788889999000011112222333344445555bbbb0002");
    let a = Note {
        value: 7_000_000_000_000,
        asset_id: 0,
        spend_pk: u64s(crate::shield::key::derive(&h, sk_a).spend_pk),
        blinding: [11, 22, 33, 44],
    };
    let b = Note {
        value: 3_000_000_000_000,
        asset_id: 0,
        spend_pk: u64s(crate::shield::key::derive(&h, sk_b).spend_pk),
        blinding: [55, 66, 77, 88],
    };
    let zero_a = Note {
        value: 0,
        asset_id: 0,
        spend_pk: [21, 22, 23, 24],
        blinding: [25, 26, 27, 28],
    };
    let zero_b = Note {
        value: 0,
        asset_id: 0,
        spend_pk: [29, 30, 31, 32],
        blinding: [33, 34, 35, 36],
    };

    let mut t = PoolTree::with_depth(h.clone(), DEPTH);
    for filler in [101u64, 202, 303] {
        t.insert([Fp::from_u64(filler); RATE]);
    }
    let ia = t.insert(commitment_of(&a));
    let ib = t.insert(commitment_of(&b));
    t.insert([Fp::from_u64(404); RATE]);
    let open_a = Witnessed {
        leaf_index: ia,
        siblings: t.path(ia).0,
    };
    let open_b = Witnessed {
        leaf_index: ib,
        siblings: t.path(ib).0,
    };

    join_split_with_paths(
        DEPTH,
        [Spend { note: &a, sk: sk_a }, Spend { note: &b, sk: sk_b }],
        [&open_a, &open_b],
        t.root(),
        [&zero_a, &zero_b],
        10_000_000_000_000,
        0,
        Break::None,
        Settle {
            clearing_price: 2_500_000,
            recipient: address_from_u64(0xDEAD_BEEF),
        },
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::witness_satisfies_public;

    /// The burner the contracts lane named as the payout address.
    const BURNER: u64 = 0x7408_ae4c;

    #[test]
    fn the_live_tree_matches_the_pools_root() {
        assert_eq!(
            hex_of(tree().root()),
            ROOT,
            "the circuit's tree is not the pool's"
        );
    }

    /// The spend satisfies its own constraints against the live root. This is
    /// the statement the box will spend an hour proving, so it is worth
    /// knowing in a second that it closes.
    #[test]
    fn the_first_spend_satisfies() {
        let js = unshield(BURNER, 1_000_000);
        assert!(
            witness_satisfies_public(&js.wired, &js.witness),
            "the first spend does not satisfy its own constraints"
        );
    }

    /// The three tests that tell the real circuit from a conservation-only
    /// fixture, asked for by the contracts lane and correctly so: the test
    /// above establishes conservation and nothing else, and a wired
    /// accumulator with a range check passes it too.
    ///
    /// Each spends the live notes and breaks exactly one property. All three
    /// must fail to satisfy; if any passes, that property is not constrained
    /// and the circuit is not proving what the pool believes it proves.
    fn live_spend_broken(brk: Break) -> crate::shield::join::JoinSplit {
        let a = note_of(A_SK, A_BLINDING, NOTE_VALUE);
        let b = note_of(B_SK, B_BLINDING, NOTE_VALUE);
        let zero_a = Note {
            value: 0,
            asset_id: 0,
            spend_pk: [1, 2, 3, 4],
            blinding: [5, 6, 7, 8],
        };
        let zero_b = Note {
            value: 0,
            asset_id: 0,
            spend_pk: [9, 10, 11, 12],
            blinding: [13, 14, 15, 16],
        };
        let t = tree();
        let open_a = Witnessed {
            leaf_index: 1,
            siblings: t.path(1).0,
        };
        let open_b = Witnessed {
            leaf_index: 2,
            siblings: t.path(2).0,
        };
        join_split_with_paths(
            DEPTH,
            [
                Spend {
                    note: &a,
                    sk: limbs(A_SK),
                },
                Spend {
                    note: &b,
                    sk: limbs(B_SK),
                },
            ],
            [&open_a, &open_b],
            limbs(ROOT),
            [&zero_a, &zero_b],
            NOTE_VALUE * 2,
            0,
            brk,
            Settle {
                clearing_price: 1_000_000,
                recipient: address_from_u64(BURNER),
            },
            None,
        )
    }

    /// Ownership. A note whose committed spend key this secret did not derive
    /// must not be spendable, or holding a commitment is enough to spend it.
    ///
    /// Built by hand rather than through `Break::NotOwner`. That variant is
    /// declared and documented in the tamper enum and applied nowhere:
    /// `nullifier_parts` handles the other four and falls through on it, and
    /// nothing else in the tree reads it. A test written against it passes
    /// because no tamper happens, which is the worst kind of green. The note
    /// below carries a spend key no secret derived, and the secret offered
    /// for it is the one that owns a different note.
    #[test]
    fn a_note_this_secret_does_not_own_is_unspendable() {
        let stolen = Note {
            value: NOTE_VALUE,
            asset_id: 0,
            spend_pk: [0xBAD, 0xBAD1, 0xBAD2, 0xBAD3],
            blinding: u64s(limbs(A_BLINDING)),
        };
        let b = note_of(B_SK, B_BLINDING, NOTE_VALUE);
        let zero_a = Note {
            value: 0,
            asset_id: 0,
            spend_pk: [1, 2, 3, 4],
            blinding: [5, 6, 7, 8],
        };
        let zero_b = Note {
            value: 0,
            asset_id: 0,
            spend_pk: [9, 10, 11, 12],
            blinding: [13, 14, 15, 16],
        };

        /*
         * The stolen note is genuinely a leaf of the tree it is proved
         * against, so membership holds and only ownership is in question.
         * Anything else would fail for the wrong reason and prove nothing
         * about ownership.
         */
        let mut tr = PoolTree::with_depth(hasher(), DEPTH);
        tr.insert(limbs(LEAF0_CM));
        let ia = tr.insert(commitment_of(&stolen));
        let ib = tr.insert(commitment_of(&b));
        let open_a = Witnessed {
            leaf_index: ia,
            siblings: tr.path(ia).0,
        };
        let open_b = Witnessed {
            leaf_index: ib,
            siblings: tr.path(ib).0,
        };

        let js = join_split_with_paths(
            DEPTH,
            [
                Spend {
                    note: &stolen,
                    sk: limbs(A_SK),
                },
                Spend {
                    note: &b,
                    sk: limbs(B_SK),
                },
            ],
            [&open_a, &open_b],
            tr.root(),
            [&zero_a, &zero_b],
            NOTE_VALUE * 2,
            0,
            Break::None,
            Settle {
                clearing_price: 1_000_000,
                recipient: address_from_u64(BURNER),
            },
            None,
        );
        assert!(
            !witness_satisfies_public(&js.wired, &js.witness),
            "a note was spent by a secret that did not derive its committed key"
        );
    }

    /// Membership. Absorbing a commitment other than the one the opening
    /// proved must fail, or a note never inserted under the declared root can
    /// be spent.
    #[test]
    fn a_note_the_opening_did_not_prove_is_unspendable() {
        let js = live_spend_broken(Break::ForeignNote);
        assert!(
            !witness_satisfies_public(&js.wired, &js.witness),
            "a commitment other than the one membership proved was spent"
        );
    }

    /// Nullifier correctness, in the form that matters: the leaf position is
    /// in the nullifier preimage, so retiring a note under a position the
    /// pool did not authenticate would let one note yield two nullifiers and
    /// be spent twice. Both the carrying form and the bit-zero form, because
    /// bit zero is the one a higher bit's binding cannot catch.
    #[test]
    fn a_note_retired_under_the_wrong_position_is_unspendable() {
        for (name, brk) in [
            ("carrying", Break::ForeignIndex),
            ("bit zero", Break::ForeignIndex0),
        ] {
            let js = live_spend_broken(brk);
            assert!(
                !witness_satisfies_public(&js.wired, &js.witness),
                "a note retired under a position the pool never authenticated ({name})"
            );
        }
    }

    /// The nullifier key must descend from the same secret as the spend key,
    /// or a commitment anyone has seen can be retired by someone who does not
    /// own it.
    #[test]
    fn a_nullifier_key_from_another_secret_is_refused() {
        let js = live_spend_broken(Break::ForeignKey);
        assert!(
            !witness_satisfies_public(&js.wired, &js.witness),
            "the nullifier key did not have to descend from the spending secret"
        );
    }

    /// A spend proved against a published association set satisfies, and the
    /// association root it declares is the one supplied rather than one the
    /// prover planted.
    ///
    /// The set here is synthetic because the registry's openings have to come
    /// from the contracts lane, but the path under test is the production one:
    /// `join_split_published`, both trees supplied, nothing planted.
    #[test]
    fn a_spend_against_a_published_association_set_satisfies() {
        let h = hasher();
        let a = note_of(A_SK, A_BLINDING, NOTE_VALUE);
        let b = note_of(B_SK, B_BLINDING, NOTE_VALUE);

        /*
         * The two notes at odd and even association positions on purpose.
         * The column this binds through used to be chosen by the bottom
         * direction bit, so a pair straddling that bit is what would have
         * caught it.
         */
        let mut at = PoolTree::with_depth(h.clone(), DEPTH);
        at.insert([Fp::from_u64(7001); RATE]);
        let ja = at.insert(commitment_of(&a));
        let jb = at.insert(commitment_of(&b));
        assert_eq!(ja % 2, 1, "one note at an odd association position");
        assert_eq!(jb % 2, 0, "the other at an even one");
        let oa = crate::shield::join::Witnessed {
            leaf_index: ja,
            siblings: at.path(ja).0,
        };
        let ob = crate::shield::join::Witnessed {
            leaf_index: jb,
            siblings: at.path(jb).0,
        };
        let assoc_root = at.root();

        let js = unshield_published([&oa, &ob], assoc_root, BURNER, 1_000_000);
        assert!(
            witness_satisfies_public(&js.wired, &js.witness),
            "a spend against a published association set does not satisfy"
        );

        /*
         * The declared association root is the supplied one. Word 1 of the
         * intent, four limbs. If this were the planted root the pool would
         * refuse the intent with UnknownAssociationRoot no matter how good
         * the proof was.
         */
        let declared: [Fp; RATE] = [js.intent[4], js.intent[5], js.intent[6], js.intent[7]];
        assert_eq!(
            declared, assoc_root,
            "the intent declares an association root the caller did not supply"
        );
    }

    /// A private transfer: nothing leaves the pool.
    ///
    /// Every other test here is an unshield, where the value exits to a named
    /// address and the amount is public by construction. A transfer is the
    /// case the system exists for: public amount zero, fee zero, and the
    /// whole value carried into two fresh output notes. What the chain sees
    /// is two nullifiers retiring and two commitments appearing. It does not
    /// see which notes were spent, what they were worth, or who now holds
    /// them.
    ///
    /// Conservation still has to close, and it closes at zero: inputs equal
    /// outputs exactly.
    #[test]
    fn a_private_transfer_satisfies_and_reveals_no_amount() {
        let a = note_of(A_SK, A_BLINDING, NOTE_VALUE);
        let b = note_of(B_SK, B_BLINDING, NOTE_VALUE);

        /*
         * The value split unevenly between the outputs on purpose: an equal
         * split could be satisfied by a circuit that only checked the total
         * twice, and the point is that the split itself is unconstrained
         * except by conservation.
         */
        let out_a = Note {
            value: NOTE_VALUE + NOTE_VALUE / 4,
            asset_id: 0,
            spend_pk: [0xA1, 0xA2, 0xA3, 0xA4],
            blinding: [0xB1, 0xB2, 0xB3, 0xB4],
        };
        let out_b = Note {
            value: NOTE_VALUE - NOTE_VALUE / 4,
            asset_id: 0,
            spend_pk: [0xC1, 0xC2, 0xC3, 0xC4],
            blinding: [0xD1, 0xD2, 0xD3, 0xD4],
        };
        assert_eq!(
            out_a.value + out_b.value,
            NOTE_VALUE * 2,
            "the transfer must conserve"
        );

        let t = tree();
        let open_a = crate::shield::join::Witnessed {
            leaf_index: 1,
            siblings: t.path(1).0,
        };
        let open_b = crate::shield::join::Witnessed {
            leaf_index: 2,
            siblings: t.path(2).0,
        };
        let js = join_split_with_paths(
            DEPTH,
            [
                Spend {
                    note: &a,
                    sk: limbs(A_SK),
                },
                Spend {
                    note: &b,
                    sk: limbs(B_SK),
                },
            ],
            [&open_a, &open_b],
            limbs(ROOT),
            [&out_a, &out_b],
            0,
            0,
            Break::None,
            Settle {
                clearing_price: 1_000_000,
                recipient: address_from_u64(0),
            },
            None,
        );
        assert!(
            witness_satisfies_public(&js.wired, &js.witness),
            "a private transfer does not satisfy its own constraints"
        );

        /*
         * The public amount really is zero in the declared intent, and the
         * declared output commitments really are the two new notes. If the
         * amount were non-zero the transfer would be an unshield wearing a
         * transfer's name, and the pool would move value.
         */
        use crate::shield::join::publics::{OUT_CM0, OUT_CM1, PUBLIC_AMOUNT};
        assert_eq!(
            js.intent[PUBLIC_AMOUNT],
            Fp::ZERO,
            "a transfer must declare no public amount"
        );
        let declared_a: [Fp; RATE] = [
            js.intent[OUT_CM0],
            js.intent[OUT_CM0 + 1],
            js.intent[OUT_CM0 + 2],
            js.intent[OUT_CM0 + 3],
        ];
        let declared_b: [Fp; RATE] = [
            js.intent[OUT_CM1],
            js.intent[OUT_CM1 + 1],
            js.intent[OUT_CM1 + 2],
            js.intent[OUT_CM1 + 3],
        ];
        assert_eq!(
            declared_a,
            commitment_of(&out_a),
            "output 0 is not the note that was created"
        );
        assert_eq!(
            declared_b,
            commitment_of(&out_b),
            "output 1 is not the note that was created"
        );
    }

    /// A transfer that creates more value than it spends must fail, which is
    /// the minting case conservation exists to stop.
    #[test]
    fn a_transfer_that_creates_value_is_refused() {
        let a = note_of(A_SK, A_BLINDING, NOTE_VALUE);
        let b = note_of(B_SK, B_BLINDING, NOTE_VALUE);
        let out_a = Note {
            value: NOTE_VALUE + 1,
            asset_id: 0,
            spend_pk: [0xA1, 0xA2, 0xA3, 0xA4],
            blinding: [0xB1, 0xB2, 0xB3, 0xB4],
        };
        let out_b = Note {
            value: NOTE_VALUE,
            asset_id: 0,
            spend_pk: [0xC1, 0xC2, 0xC3, 0xC4],
            blinding: [0xD1, 0xD2, 0xD3, 0xD4],
        };
        let t = tree();
        let open_a = crate::shield::join::Witnessed {
            leaf_index: 1,
            siblings: t.path(1).0,
        };
        let open_b = crate::shield::join::Witnessed {
            leaf_index: 2,
            siblings: t.path(2).0,
        };
        let js = join_split_with_paths(
            DEPTH,
            [
                Spend {
                    note: &a,
                    sk: limbs(A_SK),
                },
                Spend {
                    note: &b,
                    sk: limbs(B_SK),
                },
            ],
            [&open_a, &open_b],
            limbs(ROOT),
            [&out_a, &out_b],
            0,
            0,
            Break::None,
            Settle {
                clearing_price: 1_000_000,
                recipient: address_from_u64(0),
            },
            None,
        );
        assert!(
            !witness_satisfies_public(&js.wired, &js.witness),
            "a transfer minted a wei out of nothing"
        );
    }

    /// Conservation is the only thing between an unshield and minting, so
    /// claiming one wei more than the two notes hold must fail.
    #[test]
    fn claiming_more_than_the_notes_hold_is_refused() {
        let a = note_of(A_SK, A_BLINDING, NOTE_VALUE);
        let b = note_of(B_SK, B_BLINDING, NOTE_VALUE);
        let zero_a = Note {
            value: 0,
            asset_id: 0,
            spend_pk: [1, 2, 3, 4],
            blinding: [5, 6, 7, 8],
        };
        let zero_b = Note {
            value: 0,
            asset_id: 0,
            spend_pk: [9, 10, 11, 12],
            blinding: [13, 14, 15, 16],
        };
        let t = tree();
        let (a_sibs, _) = t.path(1);
        let (b_sibs, _) = t.path(2);
        let open_a = Witnessed {
            leaf_index: 1,
            siblings: a_sibs,
        };
        let open_b = Witnessed {
            leaf_index: 2,
            siblings: b_sibs,
        };
        let js = join_split_with_paths(
            DEPTH,
            [
                Spend {
                    note: &a,
                    sk: limbs(A_SK),
                },
                Spend {
                    note: &b,
                    sk: limbs(B_SK),
                },
            ],
            [&open_a, &open_b],
            limbs(ROOT),
            [&zero_a, &zero_b],
            NOTE_VALUE * 2 + 1,
            0,
            Break::None,
            Settle {
                clearing_price: 1_000_000,
                recipient: address_from_u64(BURNER),
            },
            None,
        );
        assert!(
            !witness_satisfies_public(&js.wired, &js.witness),
            "the circuit accepted a spend claiming more than its notes hold"
        );
    }
}
