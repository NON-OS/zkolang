// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{wide_mul, LIMB_BITS, N_OUT};

const P: u128 = 0xFFFF_FFFF_0000_0001;

fn recompose(out: &[u64; N_OUT]) -> u128 {
    let mut v = 0u128;
    for (k, l) in out.iter().enumerate() {
        v |= (*l as u128) << (LIMB_BITS as usize * k);
    }
    v
}

/// Why the gadget exists. A clearing product runs past the field, so reducing it
/// discards information: the true amount and the true amount plus p are
/// different fills that a field equality cannot tell apart. The relation has to
/// hold over the integers, which is what the limb schedule gives.
#[test]
fn a_clearing_product_runs_past_the_field() {
    let amount = 3_000_000_000u64;
    let price = 1_000_000_000_000_000_000u64;
    let true_product = recompose(&wide_mul(amount, price).out);

    assert!(
        true_product > P,
        "the product fits the field, so there is nothing to prove"
    );

    let other = true_product + P;
    assert_ne!(true_product, other, "distinct fills");
    assert_eq!(
        true_product % P,
        other % P,
        "same residue, indistinguishable in the field"
    );
}
