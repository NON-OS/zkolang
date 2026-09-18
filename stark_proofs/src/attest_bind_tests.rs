// NONOS Operating System (AGPL-3.0-or-later)
//! The attestation gate binds the proof to the image it admits.
//!
//! A trailer carries the sibling path and the proof; the verifier supplies the
//! root and the leaf, measuring the image it is about to run. These pin that.
//! The third test is the forgery that verified before the leaf was pinned: every
//! enrolled leaf is a public function of a shipped image, and its path ships in
//! the clear beside it, so one release image was a witness for any image under
//! any context. It has to stay refused.

use crate::crypto::stark::air::{
    build_attestation_trailer, enroll_policy_root, verify_membership_trailer, Poseidon, RATE,
};
use crate::crypto::stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// The gate's tree depth, so the shape under test is the shape that ships.
const DEPTH: usize = 8;
const EPOCH: u64 = 1;

/// The kernel's capsule context: the BLAKE3 measurement, the granted
/// capabilities, the policy epoch, each big-endian.
fn context(image: &[u8], caps: u64) -> [u8; 48] {
    let mut c = [0u8; 48];
    c[..32].copy_from_slice(blake3::hash(image).as_bytes());
    c[32..40].copy_from_slice(&caps.to_be_bytes());
    c[40..48].copy_from_slice(&EPOCH.to_be_bytes());
    c
}

fn root_bytes(root: [Fp; RATE]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, lane) in root.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    out
}

struct Set {
    hasher: Poseidon,
    images: Vec<Vec<u8>>,
    root: [u8; 32],
}

impl Set {
    /// A full tree of distinct images, measured the way the enroll tool does.
    fn enrolled() -> Set {
        let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
        let images: Vec<Vec<u8>> = (0..1usize << DEPTH)
            .map(|i| (0..1024).map(|j| (i * 31 + j) as u8).collect())
            .collect();
        let refs: Vec<&[u8]> = images.iter().map(|v| v.as_slice()).collect();
        let root = root_bytes(enroll_policy_root(&hasher, &refs));
        Set {
            hasher,
            images,
            root,
        }
    }

    fn trailer(&self, index: usize, ctx: &[u8]) -> Vec<u8> {
        let refs: Vec<&[u8]> = self.images.iter().map(|v| v.as_slice()).collect();
        build_attestation_trailer(
            &self.hasher,
            LOG_ROUNDS,
            &refs,
            index,
            ctx,
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS,
        )
    }

    fn admits(&self, root: [u8; 32], image: &[u8], trailer: &[u8], ctx: &[u8]) -> bool {
        verify_membership_trailer(
            &self.hasher,
            LOG_ROUNDS,
            root,
            image,
            DEPTH,
            trailer,
            ctx,
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS,
        )
    }
}

#[test]
fn an_enrolled_image_admits_under_its_own_context() {
    let s = Set::enrolled();
    let ctx = context(&s.images[5], 0x11);
    let t = s.trailer(5, &ctx);
    assert!(s.admits(s.root, &s.images[5], &t, &ctx));
}

#[test]
fn a_trailer_replayed_for_another_image_is_refused() {
    let s = Set::enrolled();
    let t = s.trailer(5, &context(&s.images[5], 0x11));
    let ctx = context(&s.images[6], 0x11);
    assert!(!s.admits(s.root, &s.images[6], &t, &ctx));
}

#[test]
fn an_unenrolled_image_under_an_enrolled_witness_is_refused() {
    /*
     * The attacker holds image 5 and its trailer, so they hold leaf 5 and its
     * path, and mint a fresh proof over that witness under the context of an
     * image that was never enrolled, with every capability bit set.
     */
    let s = Set::enrolled();
    let evil: Vec<u8> = b"never enrolled".repeat(100);
    let ctx = context(&evil, u64::MAX);
    let t = s.trailer(5, &ctx);
    assert!(!s.admits(s.root, &evil, &t, &ctx));
}

#[test]
fn a_neighbouring_slot_is_refused() {
    let s = Set::enrolled();
    let ctx = context(&s.images[4], 0x11);
    let t = s.trailer(5, &ctx);
    assert!(!s.admits(s.root, &s.images[4], &t, &ctx));
}

#[test]
fn a_foreign_root_is_refused() {
    let s = Set::enrolled();
    let ctx = context(&s.images[5], 0x11);
    let t = s.trailer(5, &ctx);
    let mut other = s.root;
    other[0] ^= 1;
    assert!(!s.admits(other, &s.images[5], &t, &ctx));
}

#[test]
fn the_granted_capabilities_are_bound() {
    let s = Set::enrolled();
    let t = s.trailer(5, &context(&s.images[5], 0x11));
    assert!(!s.admits(s.root, &s.images[5], &t, &context(&s.images[5], 0x13)));
}
