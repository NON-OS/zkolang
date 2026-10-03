/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A model of the integer types (section 5.1), written apart from the compiler with Rust's
 * own integer operations, for testing the reference interpreter against.
 */

use crate::prop_binary::Fail;

/** An integer type: its name, width and signedness. */
#[derive(Clone, Copy, Debug)]
pub(crate) struct Ty {
    pub name: &'static str,
    pub bits: u32,
    pub signed: bool,
}

const fn ty(name: &'static str, bits: u32, signed: bool) -> Ty {
    Ty { name, bits, signed }
}

/** Every integer type of the language. */
pub(crate) const TYPES: [Ty; 9] = [
    ty("u8", 8, false),
    ty("u16", 16, false),
    ty("u32", 32, false),
    ty("u64", 64, false),
    ty("usize", 32, false),
    ty("i8", 8, true),
    ty("i16", 16, true),
    ty("i32", 32, true),
    ty("i64", 64, true),
];

impl Ty {
    /** The smallest value. */
    pub fn min(self) -> i128 {
        match self.signed {
            true => -(1i128 << (self.bits - 1)),
            false => 0,
        }
    }

    /** The largest value. */
    pub fn max(self) -> i128 {
        match self.signed {
            true => (1i128 << (self.bits - 1)) - 1,
            false => (1i128 << self.bits) - 1,
        }
    }

    /** `v` if it is a value of the type, else an overflow. */
    pub fn fit(self, v: Option<i128>) -> Result<i128, Fail> {
        v.filter(|v| (self.min()..=self.max()).contains(v))
            .ok_or(Fail::Overflow)
    }

    /** `v` modulo 2^bits, read as a value of the type. */
    pub fn wrap(self, v: i128) -> i128 {
        let m = 1i128 << self.bits;
        let r = v.rem_euclid(m);
        if r > self.max() {
            r - m
        } else {
            r
        }
    }
}
