/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use nonos_stark::air::{Poseidon, NOTE_LIMBS, RATE};
use nonos_stark::field::Fp;

pub(super) const POOL_LOG_ROUNDS: u32 = 5;
pub(super) const SPEND_DOMAIN: u64 = 0x5350_4E44;
pub(super) const NULL_DOMAIN: u64 = 0x4E55_4C4C;

pub(super) fn tag(v: u64) -> [Fp; RATE] {
    let mut q = [Fp::ZERO; RATE];
    q[0] = Fp::from_u64(v);
    q
}

pub(super) fn secret(seed: u64) -> [Fp; RATE] {
    let mut sk = [Fp::ZERO; RATE];
    for (i, v) in sk.iter_mut().enumerate() {
        *v = Fp::from_u64(seed * 16 + i as u64 + 1);
    }
    sk
}

pub(super) fn limbs(
    value: u64,
    asset_id: u64,
    spend_pk: [Fp; RATE],
    blinding: [u64; 4],
) -> [Fp; NOTE_LIMBS] {
    let mut l = [Fp::ZERO; NOTE_LIMBS];
    l[0] = Fp::from_u64(value & 0xFFFF_FFFF);
    l[1] = Fp::from_u64(value >> 32);
    l[2] = Fp::from_u64(asset_id);
    for i in 0..4 {
        l[3 + i] = spend_pk[i];
        l[7 + i] = Fp::from_u64(blinding[i]);
    }
    l
}

pub(super) fn hasher() -> Poseidon {
    Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
}
