/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Bit patterns: shifts (section 7.5), wrapping to a width, and the conversions to and
 * from bits (section 7.9). Signed values use two's complement.
 */

use alloc::vec::Vec;

use super::FailKind;
use crate::compiler::syntax::IntTy;

/** `v` modulo `2^N`, read back as a value of `t`. */
pub(super) fn wrap(v: i128, t: IntTy) -> i128 {
    let n = t.bits();
    let m = v.rem_euclid(1i128 << n);
    if t.signed() && m >= 1i128 << (n - 1) {
        m - (1i128 << n)
    } else {
        m
    }
}

/** `a << k` or `a >> k` for a value of `t`; fails when `k` is at least the width. */
pub(super) fn shift(a: i128, k: i128, left: bool, t: IntTy) -> Result<i128, FailKind> {
    if k < 0 || k >= i128::from(t.bits()) {
        return Err(FailKind::ShiftTooFar);
    }
    /* Bits shifted past the width are lost; `>>` on a signed value keeps its sign. */
    Ok(if left { wrap(a << k, t) } else { a >> k })
}

/** The `N` bits of `v`'s pattern in `t`, least significant first. */
pub(super) fn to_bits(v: i128, n: u32) -> Vec<bool> {
    (0..n).map(|i| (v >> i) & 1 == 1).collect()
}

/** The value of `bits`, least significant first, as an unsigned integer. */
pub(super) fn from_bits(bits: &[bool]) -> i128 {
    bits.iter()
        .rev()
        .fold(0i128, |acc, &b| (acc << 1) | i128::from(b))
}
