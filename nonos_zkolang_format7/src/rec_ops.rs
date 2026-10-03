/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The arithmetic of a recorded value: each operation one entry of the tape. */

use core::ops::{Add, Mul, Sub};

use nonos_stark::field::{Felt, Fp, Fp2};

use super::rec::{constant, push, Op, Rec};
use super::rec_tape::inverted;

impl Add for Rec {
    type Output = Rec;
    fn add(self, o: Rec) -> Rec {
        push(Op::Add(self.0, o.0))
    }
}

impl Sub for Rec {
    type Output = Rec;
    fn sub(self, o: Rec) -> Rec {
        push(Op::Sub(self.0, o.0))
    }
}

impl Mul for Rec {
    type Output = Rec;
    fn mul(self, o: Rec) -> Rec {
        push(Op::Mul(self.0, o.0))
    }
}

impl Felt for Rec {
    const ZERO: Rec = Rec(0);
    const ONE: Rec = Rec(1);

    fn from_base(x: Fp) -> Rec {
        constant(Fp2::from_base(x))
    }

    /** Square and multiply, every step recorded. */
    fn pow(self, exp: u64) -> Rec {
        let (mut acc, mut base, mut e) = (Rec::ONE, self, exp);
        while e > 0 {
            if e & 1 == 1 {
                acc = acc * base;
            }
            base = base * base;
            e >>= 1;
        }
        acc
    }

    /** No image carries an inversion: the tape is marked, and `record` refuses it. */
    fn inv(self) -> Rec {
        inverted();
        Rec::ZERO
    }
}
