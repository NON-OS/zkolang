/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The program commitment: a canonical byte encoding of a compiled program and a
//! stable hash over it. On chain a proving job is posted against `commit`, the
//! 32-byte digest, so buyer and verifier agree on one exact program. Inside a
//! proof the same commitment enters as `commit_limbs`, four field elements bound
//! into the transcript, so the proof is tied to the program it claims to run.
//!
//! The encoding lives in `serialize`, the hashing in `digest`, so the shape of the
//! bytes and the shape of the commitment are each stated in one place.

mod digest;
mod encode_op;
mod serialize;

pub use digest::{commit, commit_limbs};
pub use serialize::serialize;
