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

//! The field abstraction an AIR constraint is written over, so one transition
//! serves both the base field (the composition over the coset) and the extension
//! (the out-of-domain evaluation a money-grade STARK samples at `z in Fp2`). Base
//! constants, round constants, and periodic values enter through `from_base`.

use super::element::Fp;
use super::ext::Fp2;

/// A field a constraint can be evaluated over: the base `Fp` or its extension
/// `Fp2`. Only the operations a transition and the composition need are required.
pub trait Felt:
    Copy
    + PartialEq
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;

    /// Embed a base-field element, so shared constants live in one place.
    fn from_base(x: Fp) -> Self;

    /// Exponentiation, for domain vanishing polynomials.
    fn pow(self, exp: u64) -> Self;

    /// The S-box, `x^7`. It is the hash's inner loop, so it is an addition
    /// chain rather than the square and multiply loop: `x^2`, `x^3`, `x^6`,
    /// `x^7` is four multiplications where the generic form spends six and a
    /// branch per bit. Same value, and every implementation must keep it so.
    #[inline]
    fn sbox7(self) -> Self {
        let x2 = self * self;
        let x3 = x2 * self;
        let x6 = x3 * x3;
        x6 * self
    }

    /// Multiplicative inverse; the caller guarantees a nonzero argument, as the
    /// off-domain evaluation points do.
    fn inv(self) -> Self;
}

impl Felt for Fp {
    const ZERO: Fp = Fp::ZERO;
    const ONE: Fp = Fp::ONE;

    #[inline]
    fn from_base(x: Fp) -> Fp {
        x
    }

    #[inline]
    fn pow(self, exp: u64) -> Fp {
        Fp::pow(self, exp)
    }

    #[inline]
    fn inv(self) -> Fp {
        Fp::inv(self)
    }
}

impl Felt for Fp2 {
    const ZERO: Fp2 = Fp2::ZERO;
    const ONE: Fp2 = Fp2::ONE;

    #[inline]
    fn from_base(x: Fp) -> Fp2 {
        Fp2::from_base(x)
    }

    #[inline]
    fn pow(self, exp: u64) -> Fp2 {
        Fp2::pow(self, exp)
    }

    #[inline]
    fn inv(self) -> Fp2 {
        Fp2::inv(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The chain against the exponentiation it replaces, over both fields. The
    /// generic form is the specification: the S-box is the hash's inner loop
    /// and a wrong value there moves every digest in the system.
    #[test]
    fn the_sbox_agrees_with_the_seventh_power() {
        let mut x = Fp::from_u64(0x9e37_79b9_7f4a_7c15);
        for _ in 0..20_000 {
            x = x * x + Fp::ONE;
            assert_eq!(Felt::sbox7(x), Fp::pow(x, 7));
            let e = Fp2 { c0: x, c1: x + Fp::ONE };
            assert_eq!(Felt::sbox7(e), Felt::pow(e, 7));
        }
    }
}
