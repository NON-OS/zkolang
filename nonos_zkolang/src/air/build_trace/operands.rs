/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Set the operand columns an opcode carries.

use nonos_stark::field::Fp;

use super::super::layout::{A, AUX, B, C, D, IMM};
use super::super::step_air::StepAir;
use crate::trace::{OpTag, Row};

impl StepAir {
    pub(super) fn write_operands(flat: &mut [Fp], base: usize, row: &Row) {
        match row.op {
            OpTag::Imm | OpTag::Inp => {
                flat[base + D] = row.rd;
                flat[base + IMM] = row.imm;
            }
            OpTag::Add | OpTag::Sub | OpTag::Mul | OpTag::Eq => {
                flat[base + A] = row.ra;
                flat[base + B] = row.rb;
                flat[base + D] = row.rd;
                if matches!(row.op, OpTag::Eq) {
                    flat[base + AUX] = row.aux;
                }
            }
            OpTag::Inv => {
                flat[base + A] = row.ra;
                flat[base + D] = row.rd;
                flat[base + AUX] = row.aux;
            }
            OpTag::Sel => {
                flat[base + A] = row.ra;
                flat[base + B] = row.rb;
                flat[base + C] = row.rc;
                flat[base + D] = row.rd;
            }
            OpTag::Bool | OpTag::Assert | OpTag::Out => flat[base + A] = row.ra,
            OpTag::Halt => {}
        }
    }
}
