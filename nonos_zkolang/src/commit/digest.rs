/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The commitment itself: a blake3 digest over the canonical encoding, offered two
//! ways. `commit` is the 32-byte value the on-chain market posts jobs against;
//! `commit_limbs` is the same 32 bytes as four field elements, for binding the
//! program into a proof's public statement.

use nonos_stark::field::Fp;
use nonos_stark::hash::blake3_hash;

use super::serialize;
use crate::isa::Op;

/// The 32-byte program commitment, a blake3 digest of the canonical encoding.
/// This is the `programCommit` a proving job is posted against on chain.
pub fn commit(program: &[Op]) -> [u8; 32] {
    blake3_hash(&serialize(program))
}

/// The commitment as four field elements, for binding the program into a proof's
/// public statement. Each limb is eight bytes of the digest reduced into the
/// field, so the four limbs carry the whole 32-byte commitment.
pub fn commit_limbs(program: &[Op]) -> [Fp; 4] {
    let h = commit(program);
    let mut limbs = [Fp::ZERO; 4];
    for (i, limb) in limbs.iter_mut().enumerate() {
        let mut b = [0u8; 8];
        b.copy_from_slice(&h[i * 8..i * 8 + 8]);
        *limb = Fp::from_u64(u64::from_le_bytes(b));
    }
    limbs
}
