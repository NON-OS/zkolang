/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Replaying an instruction that computes an SSA value, constraint or output. */

use super::verify_op::Replay;
use crate::compiler::ssa::{Inst, V};
use crate::isa::Op;

impl<'a> Replay<'a> {
    /** The instruction computing `v`. */
    pub(super) fn def(&mut self, op: Op, v: V) -> Result<(), &'static str> {
        let inst = *self.ssa.get(v).ok_or("no such value")?;
        let h = |r: u8, x: V| self.regs.get(usize::from(r)).copied().flatten() == Some(x);
        let (ok, d) = match (inst, op) {
            (Inst::Const(_) | Inst::Input(_) | Inst::Advice(_), _) => {
                return self
                    .reload(op, v)
                    .map(|()| self.mark(v))
                    .map_err(|_| "a value read wrongly");
            }
            (Inst::Add(a, b), Op::Add { d, a: x, b: y }) => (h(x, a) && h(y, b), Some(d)),
            (Inst::Sub(a, b), Op::Sub { d, a: x, b: y }) => (h(x, a) && h(y, b), Some(d)),
            (Inst::Mul(a, b), Op::Mul { d, a: x, b: y }) => (h(x, a) && h(y, b), Some(d)),
            (Inst::Eq(a, b), Op::Eq { d, a: x, b: y }) => (h(x, a) && h(y, b), Some(d)),
            (Inst::Inv(a), Op::Inv { d, a: x }) => (h(x, a), Some(d)),
            (
                Inst::Sel(c, a, b),
                Op::Sel {
                    d,
                    c: z,
                    a: x,
                    b: y,
                },
            ) => (h(z, c) && h(x, a) && h(y, b), Some(d)),
            (Inst::AssertBool(a), Op::Bool { a: x }) => (h(x, a), None),
            (Inst::AssertZero(a), Op::Assert { a: x }) => (h(x, a), None),
            (Inst::Output(i, a), Op::Out { a: x, idx }) => (h(x, a) && idx == i, None),
            _ => (false, None),
        };
        if !ok {
            return Err("an instruction does not compute its value from its operands");
        }
        if let Some(r) = d.and_then(|d| self.regs.get_mut(usize::from(d))) {
            *r = Some(v);
        }
        self.mark(v);
        Ok(())
    }

    /** Record that `v` is computed. */
    fn mark(&mut self, v: V) {
        if let Some(x) = self.done.get_mut(v.index()) {
            *x = true;
        }
    }
}
