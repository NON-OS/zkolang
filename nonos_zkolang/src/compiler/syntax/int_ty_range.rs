/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The width, signedness and value range of each integer type. */

use super::int_ty::IntTy;

impl IntTy {
    /** The width in bits. */
    pub fn bits(self) -> u32 {
        match self {
            IntTy::U8 | IntTy::I8 => 8,
            IntTy::U16 | IntTy::I16 => 16,
            IntTy::U32 | IntTy::I32 | IntTy::Usize => 32,
            IntTy::U64 | IntTy::I64 => 64,
        }
    }

    /** Whether the type is signed. */
    pub fn signed(self) -> bool {
        matches!(self, IntTy::I8 | IntTy::I16 | IntTy::I32 | IntTy::I64)
    }

    /** The smallest value, as a mathematical integer. */
    pub fn min(self) -> i128 {
        if self.signed() {
            -(1i128 << (self.bits() - 1))
        } else {
            0
        }
    }

    /** The largest value, as a mathematical integer. */
    pub fn max(self) -> i128 {
        if self.signed() {
            (1i128 << (self.bits() - 1)) - 1
        } else {
            (1i128 << self.bits()) - 1
        }
    }

    /** Whether `v` is a value of the type. */
    pub fn contains(self, v: i128) -> bool {
        self.min() <= v && v <= self.max()
    }

    /** Whether every value of `self` is a value of `to`. */
    pub fn fits_in(self, to: IntTy) -> bool {
        to.min() <= self.min() && self.max() <= to.max()
    }
}
