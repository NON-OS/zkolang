/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Ordered comparison, `a < b`. Both operands are range proven to sixteen bits, then
//! the difference `a + 2^16 - b` lands in `[1, 2^17)` and is decomposed into seventeen
//! bits; its top bit is one exactly when `a >= b`, so `a < b` is its complement. The
//! decompositions supply their bits as advice the driver fills from the run, and the
//! range proofs are what make the comparison sound: without them a value outside the
//! sixteen-bit range could forge an order.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

/// The comparison operand width in bits. Operands must lie in `[0, 2^16)`.
const W: u8 = 16;

impl Compiler {
    pub(crate) fn compare(&mut self, l: &Expr, r: &Expr) -> Result<Val, CompileError> {
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        let ta = self.decompose(a.reg, W)?;
        self.free.push(ta);
        let tb = self.decompose(b.reg, W)?;
        self.free.push(tb);

        let base = self.emit_num(1u64 << W)?;
        let sub = self.alloc()?;
        self.ops.push(Op::Sub {
            d: sub,
            a: a.reg,
            b: b.reg,
        });
        self.release(&a);
        self.release(&b);
        let t = self.alloc()?;
        self.ops.push(Op::Add {
            d: t,
            a: sub,
            b: base.reg,
        });
        self.free.push(sub);
        self.free.push(base.reg);

        let sign = self.decompose(t, W + 1)?;
        self.free.push(t);
        let one = self.emit_num(1)?;
        self.free.push(sign);
        self.free.push(one.reg);
        let d = self.alloc()?;
        self.ops.push(Op::Sub {
            d,
            a: one.reg,
            b: sign,
        });
        Ok(Val { reg: d, temp: true })
    }
}
