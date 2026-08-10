// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;

/// Proposed in SPEC.md section 4, pending wallet ratification. The wallet and the
/// client must derive against the same values or every note is unspendable.
pub(crate) const SPEND_DOMAIN: u64 = 0x5350_4E44;
pub(crate) const NULL_DOMAIN: u64 = 0x4E55_4C4C;

pub(crate) fn tag(v: u64) -> [Fp; RATE] {
    let mut q = [Fp::ZERO; RATE];
    q[0] = Fp::from_u64(v);
    q
}
