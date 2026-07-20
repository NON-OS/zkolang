/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The canonical byte encoding of a program: a version byte followed by one
//! fixed-width record per opcode, in order. It is versioned and fixed-width so the
//! encoding is reproducible across compilers and stable across time.

use alloc::vec::Vec;

use super::encode_op::encode_op;
use crate::isa::Op;

/// The encoding version. Bump it if the opcode encoding ever changes, so digests from
/// different encodings never collide.
const VERSION: u8 = 1;

/// Canonically serialize a program to bytes: a version byte then one record per op.
pub fn serialize(program: &[Op]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.push(VERSION);
    for op in program {
        encode_op(*op, &mut out);
    }
    out
}
