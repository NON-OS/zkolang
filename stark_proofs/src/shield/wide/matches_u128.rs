// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{wide_mul, LIMB_BITS, N_OUT};

fn recompose(out: &[u64; N_OUT]) -> u128 {
    let mut v = 0u128;
    for (k, l) in out.iter().enumerate() {
        v |= (*l as u128) << (LIMB_BITS as usize * k);
    }
    v
}

/// The limb and carry schedule must equal the true 128 bit product, including at
/// the bounds where every carry is live.
#[test]
fn the_limb_schedule_is_the_true_product() {
    let cases = [
        (0u64, 0u64),
        (1, 1),
        (u64::MAX, 1),
        (u64::MAX, u64::MAX),
        (1_000_000_000_000_000_000, 3_141_592_653_589_793),
        (0xFFFF_FFFF, 0xFFFF_FFFF),
        (12_345_678_901_234_567, 987_654_321),
    ];
    for (a, b) in cases {
        let p = wide_mul(a, b);
        assert_eq!(recompose(&p.out), (a as u128) * (b as u128), "{a} x {b}");
    }
}

/// Every limb is a limb. If any exceeded the base the reassembly would be
/// ambiguous, which is what an unrange checked decomposition allows.
#[test]
fn every_limb_stays_within_the_base() {
    let p = wide_mul(u64::MAX, u64::MAX);
    for l in p.out {
        assert!(l < 1u64 << LIMB_BITS);
    }
}
