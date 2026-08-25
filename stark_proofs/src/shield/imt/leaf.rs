// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;

/// Nullifier set leaf: the key, the neighbour it points at, and whether it is the
/// last. Ten limbs, the tag at the first past the payload, hashed by the same
/// three-compression tree as a note commitment.
///
/// `is_last` replaces a magic maximum. A sentinel value would have to be
/// non-canonical to sit above every key, and a non-canonical value does not
/// survive the reduction the hash applies, so the leaf would commit to something
/// the comparison never sees. A flag is injective by construction.
pub(crate) const IMT_LEAF_DOMAIN: u64 = 0x494D_544C;

/// Limbs the payload occupies, before the tag.
pub(crate) const IMT_LEAF_LIMBS: usize = 10;

#[derive(Clone, Copy)]
pub(crate) struct Leaf {
    /// The nullifier, four limbs, little endian, canonical.
    pub value: [Fp; RATE],
    pub next_index: u64,
    /// Zero when `is_last`, which the constraint requires rather than assumes.
    pub next_value: [Fp; RATE],
    pub is_last: bool,
}

impl Leaf {
    /// The empty set: one leaf below every key, pointing nowhere.
    pub fn sentinel() -> Leaf {
        Leaf {
            value: [Fp::ZERO; RATE],
            next_index: 0,
            next_value: [Fp::ZERO; RATE],
            is_last: true,
        }
    }

    /// value, nextValue, nextIndex, isLast, then the tag. The two keys occupy the
    /// first two quads so a compression takes them whole.
    pub fn limbs(&self) -> [Fp; 16] {
        let mut l = [Fp::ZERO; 16];
        l[..RATE].copy_from_slice(&self.value);
        l[RATE..2 * RATE].copy_from_slice(&self.next_value);
        l[2 * RATE] = Fp::from_u64(self.next_index);
        l[IMT_LEAF_LIMBS - 1] = Fp::from_u64(self.is_last as u64);
        l[IMT_LEAF_LIMBS] = Fp::from_u64(IMT_LEAF_DOMAIN);
        l
    }

    pub fn quads(&self) -> [[Fp; RATE]; 4] {
        let l = self.limbs();
        let mut q = [[Fp::ZERO; RATE]; 4];
        for (i, qi) in q.iter_mut().enumerate() {
            qi.copy_from_slice(&l[i * RATE..(i + 1) * RATE]);
        }
        q
    }
}
