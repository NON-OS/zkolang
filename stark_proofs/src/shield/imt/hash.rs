// NONOS Operating System (AGPL-3.0-or-later)

use super::leaf::Leaf;
use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::note::POOL_LOG_ROUNDS;

pub fn hasher() -> Poseidon {
    Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
}

/// The leaf hash: the two keys in the first compression, the rest in the second,
/// and those two compressed together. Same three-compress tree as a note
/// commitment, so the contract runs one hasher for both trees.
pub fn leaf_hash(h: &Poseidon, leaf: &Leaf) -> [Fp; RATE] {
    let l = leaf.limbs();
    let quad = |i: usize| {
        let mut q = [Fp::ZERO; RATE];
        q.copy_from_slice(&l[i * RATE..(i + 1) * RATE]);
        q
    };
    let d0 = h.compress(&quad(0), &quad(1));
    let d1 = h.compress(&quad(2), &quad(3));
    h.compress(&d0, &d1)
}

/// The empty slot is a real leaf, so the zeros chain is based on its hash rather
/// than on nothing.
pub fn empty_leaf(h: &Poseidon) -> [Fp; RATE] {
    leaf_hash(
        h,
        &Leaf {
            value: [Fp::ZERO; RATE],
            next_index: 0,
            next_value: [Fp::ZERO; RATE],
            is_last: false,
        },
    )
}

/// The root of a tree holding one sentinel and empty slots to `depth`.
pub fn genesis_root(h: &Poseidon, depth: usize) -> [Fp; RATE] {
    let mut zero = empty_leaf(h);
    let mut node = leaf_hash(h, &Leaf::sentinel());
    for _ in 0..depth {
        node = h.compress(&node, &zero);
        zero = h.compress(&zero, &zero);
    }
    node
}

/// Four limbs as one 256 bit word, little endian, which is the order the contract
/// packs and compares in.
pub fn pack(d: &[Fp; RATE]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, v) in d.iter().enumerate() {
        out[i * 8..(i + 1) * 8].copy_from_slice(&v.value().to_le_bytes());
    }
    out
}

pub fn hex(b: &[u8; 32]) -> alloc::string::String {
    use core::fmt::Write;
    let mut s = alloc::string::String::from("0x");
    for x in b.iter().rev() {
        let _ = write!(s, "{x:02x}");
    }
    s
}
