// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Leg, ValueBalance};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(crate) const LOG_T: u32 = 3;

pub(crate) fn limbs_of(v: u64) -> (Fp, Fp) {
    (Fp::from_u64(v & 0xFFFF_FFFF), Fp::from_u64(v >> 32))
}

/// Row order the legs declare: two spent notes, two created, then the public
/// amount and the fee.
pub(crate) fn balance(values: &[u64; 4], public_amount: u64, fee: u64) -> (ValueBalance, Vec<Fp>) {
    let mut terms: Vec<(Fp, Fp)> = values.iter().map(|v| limbs_of(*v)).collect();
    terms.push(limbs_of(public_amount));
    terms.push(limbs_of(fee));
    let legs = alloc::vec![
        Leg::Input,
        Leg::Input,
        Leg::Output,
        Leg::Output,
        Leg::Output,
        Leg::Output,
        Leg::Pad,
        Leg::Pad,
    ];
    let air = ValueBalance { log_t: LOG_T, legs };
    let trace = air.trace(&terms);
    (air, trace)
}
