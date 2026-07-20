/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The zero-or-one test.

use nonos_stark::field::Fp;

/// True when a field element is zero or one, the check the boolean and select opcodes
/// gate on.
pub(super) fn is_bool(v: Fp) -> bool {
    v == Fp::ZERO || v == Fp::ONE
}
