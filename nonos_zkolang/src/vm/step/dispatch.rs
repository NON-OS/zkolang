/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The instruction dispatcher.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::super::{ProveError, Vm};
use crate::isa::Op;
use crate::trace::{OpTag, Row};

impl Vm {
    /// Execute one instruction, filling `row`. Returns `Ok(true)` on `Halt`.
    pub(in crate::vm) fn step(
        &mut self,
        op: Op,
        inputs: &[Fp],
        outputs: &mut Vec<Fp>,
        row: &mut Row,
        clk: u64,
    ) -> Result<bool, ProveError> {
        match op {
            Op::Imm { d, v } => self.step_imm(d, v, row)?,
            Op::Add { d, a, b } => self.arith(OpTag::Add, d, a, b, row, |x, y| x + y)?,
            Op::Sub { d, a, b } => self.arith(OpTag::Sub, d, a, b, row, |x, y| x - y)?,
            Op::Mul { d, a, b } => self.arith(OpTag::Mul, d, a, b, row, |x, y| x * y)?,
            Op::Inv { d, a } => self.step_inv(d, a, row, clk)?,
            Op::Sel { d, c, a, b } => self.step_sel(d, c, a, b, row, clk)?,
            Op::Eq { d, a, b } => self.step_eq(d, a, b, row)?,
            Op::Bool { a } => self.step_bool(a, row, clk)?,
            Op::Assert { a } => self.step_assert(a, row, clk)?,
            Op::Inp { d, idx } => self.step_inp(d, idx, inputs, row)?,
            Op::Out { a, idx } => self.step_out(a, idx, outputs, row)?,
            Op::Halt => {
                row.op = OpTag::Halt;
                return Ok(true);
            }
        }
        Ok(false)
    }
}
