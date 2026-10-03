/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Caller inputs as field elements, refusing any the field cannot hold. */

use alloc::vec::Vec;

use nonos_stark::field::{Fp, P};

use super::RunError;

/**
 * The public inputs, then the secrets, as field elements. A value at or above the modulus
 * is refused rather than reduced, since reducing it would prove a statement about a
 * different number than the caller passed.
 */
pub(super) fn field_inputs(public: &[u64], secret: &[u64]) -> Result<Vec<Fp>, RunError> {
    public
        .iter()
        .chain(secret)
        .enumerate()
        .map(|(position, &v)| match v {
            v if v >= P => Err(RunError::InputNotInField { position }),
            v => Ok(Fp::from_u64(v)),
        })
        .collect()
}
