// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{NOTE_DOMAIN, NOTE_LIMBS, RATE};
use crate::crypto::stark::field::Fp;

/// The pool hash: 1 << 5 rounds, matching FULL_ROUNDS in PoseidonGoldilocks.sol.
pub(crate) const POOL_LOG_ROUNDS: u32 = 5;

#[derive(Clone, Copy)]
pub(crate) struct Note {
    pub value: u64,
    pub asset_id: u64,
    pub spend_pk: [u64; 4],
    pub blinding: [u64; 4],
}

impl Note {
    /// Limb order is ShieldedPool::_computeCommitment: value low then high, asset
    /// id, then the two digests. spend_pk occupies limbs 3 through 6.
    pub fn limbs(&self) -> [Fp; NOTE_LIMBS] {
        let mut l = [Fp::ZERO; NOTE_LIMBS];
        l[0] = Fp::from_u64(self.value & 0xFFFF_FFFF);
        l[1] = Fp::from_u64(self.value >> 32);
        l[2] = Fp::from_u64(self.asset_id);
        for i in 0..4 {
            l[3 + i] = Fp::from_u64(self.spend_pk[i]);
            l[7 + i] = Fp::from_u64(self.blinding[i]);
        }
        l
    }
}

pub(crate) fn quads(limbs: &[Fp; NOTE_LIMBS]) -> [[Fp; RATE]; 4] {
    let mut p = [Fp::ZERO; 16];
    p[..NOTE_LIMBS].copy_from_slice(limbs);
    p[NOTE_LIMBS] = Fp::from_u64(NOTE_DOMAIN);
    let mut q = [[Fp::ZERO; RATE]; 4];
    for (i, qi) in q.iter_mut().enumerate() {
        qi.copy_from_slice(&p[i * RATE..(i + 1) * RATE]);
    }
    q
}
