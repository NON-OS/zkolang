// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;

/// An EVM address, the twenty bytes as they appear on chain, most significant
/// first.
pub type Address = [u8; 20];

/// Terms the circuit does not compute: the settlement destination and the batch
/// clearing price. They are
/// inputs to the statement rather than outputs of it, so a boundary is the whole
/// binding: the proof is void for any other value, which is what stops a settler
/// substituting one. Price uniformity across a batch is a separate constraint
/// and lands with the batch assembly.
#[derive(Clone, Copy)]
pub struct Settle {
    pub clearing_price: u64,
    pub recipient: Address,
}

/// The address as the four Goldilocks limbs the intent carries: limb 0 the
/// low 64 bits of the address read as a 160 bit integer, limb 1 the next 64,
/// limb 2 the top 32, limb 3 zero. The same rule the chain applies to a
/// 256 bit word, applied to a 160 bit one, so both sides derive the same
/// list from the same address and neither has to carry a second encoding.
pub fn address_limbs(a: &Address) -> [Fp; RATE] {
    let mut low = [0u8; 8];
    low.copy_from_slice(&a[12..20]);
    let mut mid = [0u8; 8];
    mid.copy_from_slice(&a[4..12]);
    let mut top = [0u8; 8];
    top[4..8].copy_from_slice(&a[0..4]);
    [
        Fp::from_u64(u64::from_be_bytes(low)),
        Fp::from_u64(u64::from_be_bytes(mid)),
        Fp::from_u64(u64::from_be_bytes(top)),
        Fp::ZERO,
    ]
}

/// An address whose value is a small integer, for fixtures that name a
/// recipient rather than pay one. `0xBEEF` is the address with `0xBEEF` in
/// its low bytes and zeros above.
pub const fn address_from_u64(v: u64) -> Address {
    let b = v.to_be_bytes();
    let mut a = [0u8; 20];
    let mut i = 0;
    while i < 8 {
        a[12 + i] = b[i];
        i += 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_limbs_of_an_address_split_at_64_bit_boundaries() {
        let mut a = [0u8; 20];
        for (i, b) in a.iter_mut().enumerate() {
            *b = 0xA0 + i as u8;
        }
        let l = address_limbs(&a);
        assert_eq!(l[0].to_u64(), 0xACAD_AEAF_B0B1_B2B3);
        assert_eq!(l[1].to_u64(), 0xA4A5_A6A7_A8A9_AAAB);
        assert_eq!(l[2].to_u64(), 0x0000_0000_A0A1_A2A3);
        assert_eq!(l[3].to_u64(), 0);
    }

    #[test]
    fn a_small_address_lands_in_the_low_limb() {
        let l = address_limbs(&address_from_u64(0xBEEF));
        assert_eq!(l[0].to_u64(), 0xBEEF);
        assert!(l[1..].iter().all(|v| v.to_u64() == 0));
    }
}
