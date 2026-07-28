// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The field ring operations. Multiplication reduces through a `u128` modulo;
//! the optimized special-form reduction is a later refinement held to the same
//! specification the property proofs fix.

use super::element::{Fp, EPSILON, P};

impl core::ops::Add for Fp {
    type Output = Fp;

    #[inline]
    fn add(self, other: Fp) -> Fp {
        let (sum, over) = self.0.overflowing_add(other.0);
        // a + b lies in [0, 2P). On overflow past 2^64 the field result is
        // sum + (2^64 - P) = sum + EPSILON, already canonical. Otherwise one
        // conditional subtraction canonicalizes.
        let mut r = sum;
        if over {
            r = r.wrapping_add(EPSILON);
        } else if r >= P {
            r -= P;
        }
        Fp(r)
    }
}

impl core::ops::Sub for Fp {
    type Output = Fp;

    #[inline]
    fn sub(self, other: Fp) -> Fp {
        let (diff, borrow) = self.0.overflowing_sub(other.0);
        // On borrow the wrapped value is a - b + 2^64; the field result a - b + P
        // is that minus (2^64 - P) = minus EPSILON.
        let r = if borrow {
            diff.wrapping_sub(EPSILON)
        } else {
            diff
        };
        Fp(r)
    }
}

impl core::ops::Neg for Fp {
    type Output = Fp;

    #[inline]
    fn neg(self) -> Fp {
        if self.0 == 0 {
            Fp(0)
        } else {
            Fp(P - self.0)
        }
    }
}

impl core::ops::Mul for Fp {
    type Output = Fp;

    #[inline]
    fn mul(self, other: Fp) -> Fp {
        Fp(reduce128((self.0 as u128) * (other.0 as u128)))
    }
}

/// Reduce a 128-bit product into the canonical range using the Goldilocks special
/// form, with no division. Writing the product as
/// `x = x_lo + x_hi_lo * 2^64 + x_hi_hi * 2^96` and using `2^64 = 2^32 - 1` and
/// `2^96 = -1` modulo `p`, it reduces to `x_lo - x_hi_hi + x_hi_lo * (2^32 - 1)`.
/// Each of the two steps corrects a carry worth `2^64 = EPSILON` modulo `p`, and a
/// final conditional subtraction canonicalizes. This is the reduction the module's
/// property proofs pin to the plain `u128` modulo it replaces.
#[inline]
fn reduce128(x: u128) -> u64 {
    let x_lo = x as u64;
    let x_hi = (x >> 64) as u64;
    let x_hi_hi = x_hi >> 32;
    let x_hi_lo = x_hi & EPSILON;

    // x_lo - x_hi_hi. A borrow means the wrap added 2^64, worth EPSILON modulo p,
    // so take it back off; this cannot underflow.
    let (mut t0, borrow) = x_lo.overflowing_sub(x_hi_hi);
    if borrow {
        t0 = t0.wrapping_sub(EPSILON);
    }
    // x_hi_lo * (2^32 - 1), at most (2^32 - 1)^2, which fits in a u64.
    let t1 = x_hi_lo * EPSILON;
    // t0 + t1. The sum stays below 2^64 + p, so a carry is worth exactly EPSILON and
    // adding it back cannot overflow a second time.
    let (mut t2, carry) = t0.overflowing_add(t1);
    if carry {
        t2 = t2.wrapping_add(EPSILON);
    }
    if t2 >= P {
        t2 - P
    } else {
        t2
    }
}

#[cfg(test)]
mod tests {
    use super::P;
    use crate::field::element::Fp;

    // The specification the fast reduction is held to: the plain 128-bit modulo.
    fn reference(a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % (P as u128)) as u64
    }

    fn check(a: u64, b: u64) {
        let ca = Fp::from_u64(a).value();
        let cb = Fp::from_u64(b).value();
        let got = (Fp::from_u64(a) * Fp::from_u64(b)).value();
        assert!(got < P, "product {ca} * {cb} left the canonical range: {got}");
        assert_eq!(got, reference(ca, cb), "product {ca} * {cb}");
    }

    #[test]
    fn fast_reduction_agrees_with_the_u128_modulo() {
        // Boundary operands where the carry corrections bite: zero, one, the top of the
        // field, powers of two either side of the 2^32 and 2^64 splits, and EPSILON.
        let edges = [
            0u64,
            1,
            2,
            P - 1,
            P - 2,
            0xFFFF_FFFF,
            1u64 << 32,
            (1u64 << 32) + 1,
            1u64 << 63,
            0xFFFF_FFFF_FFFF_FFFF,
            P + 1,
        ];
        for &a in &edges {
            for &b in &edges {
                check(a, b);
            }
        }

        // A large deterministic sweep. splitmix64 keeps it reproducible, and operands
        // near 2^64 stress the high limbs the reduction folds down.
        let mut s = 0x1234_5678_9abc_def0u64;
        let mut next = || {
            s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        for _ in 0..3_000_000 {
            check(next(), next());
        }
    }
}
