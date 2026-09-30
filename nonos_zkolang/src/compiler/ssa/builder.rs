/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Writing a program instruction by instruction. The builder stops growing a program past
 * a bound, well beyond the largest provable one, so lowering a huge unrolling ends fast;
 * the caller reports it.
 */

use nonos_stark::field::P;

use super::{Inst, Ssa, V};

/** The most instructions a program may grow to before lowering gives up. */
pub const MAX_INSTS: usize = 1 << 22;

/** A program being written. */
#[derive(Debug, Default)]
pub struct Builder {
    pub ssa: Ssa,
    /** Whether the program outgrew `MAX_INSTS`; what was written after is dropped. */
    pub over: bool,
}

impl Builder {
    /** Append `i` and return the value it defines. */
    pub fn emit(&mut self, i: Inst) -> V {
        if self.ssa.insts.len() >= MAX_INSTS {
            self.over = true;
            return V(0);
        }
        let v = V(u32::try_from(self.ssa.insts.len()).unwrap_or(u32::MAX));
        self.ssa.insts.push(i);
        v
    }

    /** The constant `x mod p`, for any integer `x`. */
    pub fn konst(&mut self, x: i128) -> V {
        let r = x.rem_euclid(i128::from(P));
        self.emit(Inst::Const(u64::try_from(r).unwrap_or(0)))
    }

    pub fn add(&mut self, a: V, b: V) -> V {
        self.emit(Inst::Add(a, b))
    }

    pub fn sub(&mut self, a: V, b: V) -> V {
        self.emit(Inst::Sub(a, b))
    }

    pub fn mul(&mut self, a: V, b: V) -> V {
        self.emit(Inst::Mul(a, b))
    }

    /** `c ? a : b`. */
    pub fn sel(&mut self, c: V, a: V, b: V) -> V {
        if a == b {
            return a;
        }
        self.emit(Inst::Sel(c, a, b))
    }

    /** `1 - b` for a boolean `b`. */
    pub fn not(&mut self, b: V) -> V {
        let one = self.konst(1);
        self.sub(one, b)
    }
}
