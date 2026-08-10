// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;

/// Published in spec/shield-key-hierarchy.json, which the wallet and the client
/// derive against. Changing either value changes every note, so amend the spec
/// and regenerate the vector rather than editing here.
pub(crate) const SPEND_DOMAIN: u64 = 0x5350_4E44;
pub(crate) const NULL_DOMAIN: u64 = 0x4E55_4C4C;

pub(crate) fn tag(v: u64) -> [Fp; RATE] {
    let mut q = [Fp::ZERO; RATE];
    q[0] = Fp::from_u64(v);
    q
}
