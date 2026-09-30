/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Rewriting the values an instruction reads. */

use super::{Hint, Inst, V};

impl Inst {
    /** The instruction with each operand `v`, its hint's too, replaced by `f(v)`. */
    pub fn map(self, f: &mut dyn FnMut(V) -> V) -> Inst {
        match self {
            Inst::Const(_) | Inst::Input(_) => self,
            Inst::Advice(h) => Inst::Advice(match h {
                Hint::Bit(a, k) => Hint::Bit(f(a), k),
                Hint::Quot(a, b) => Hint::Quot(f(a), f(b)),
                Hint::Rem(a, b) => Hint::Rem(f(a), f(b)),
                Hint::Copy(a) => Hint::Copy(f(a)),
            }),
            Inst::Add(a, b) => Inst::Add(f(a), f(b)),
            Inst::Sub(a, b) => Inst::Sub(f(a), f(b)),
            Inst::Mul(a, b) => Inst::Mul(f(a), f(b)),
            Inst::Inv(a) => Inst::Inv(f(a)),
            Inst::Sel(c, a, b) => Inst::Sel(f(c), f(a), f(b)),
            Inst::Eq(a, b) => Inst::Eq(f(a), f(b)),
            Inst::AssertBool(a) => Inst::AssertBool(f(a)),
            Inst::AssertZero(a) => Inst::AssertZero(f(a)),
            Inst::Output(i, a) => Inst::Output(i, f(a)),
            Inst::RangeCheck(a, n) => Inst::RangeCheck(f(a), n),
            Inst::Bit(a, k, n) => Inst::Bit(f(a), k, n),
            Inst::FieldBit(a, k) => Inst::FieldBit(f(a), k),
            Inst::Quot(a, b, n) => Inst::Quot(f(a), f(b), n),
            Inst::Rem(a, b, n) => Inst::Rem(f(a), f(b), n),
        }
    }
}
