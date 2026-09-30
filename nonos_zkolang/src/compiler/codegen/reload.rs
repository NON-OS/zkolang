/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Bringing a value into a register: it is there, or it is read again. */

use nonos_stark::field::Fp;

use super::machine::{CodegenError, Origin};
use super::state::Cg;
use crate::compiler::ssa::{Inst, V};
use crate::isa::Op;

impl<'a> Cg<'a> {
    /** The register holding `v`, reading it again if none does; `keep` stays in place. */
    pub(super) fn ensure(&mut self, v: V, keep: &[V]) -> Result<u8, CodegenError> {
        if let Some(Some(r)) = self.reg.get(v.index()) {
            return Ok(*r);
        }
        let d = self.take_reg(keep)?;
        let op = match (self.ssa.get(v), self.slot.get(v.index()).copied().flatten()) {
            (Some(Inst::Const(c)), _) => Op::Imm {
                d,
                v: Fp::from_u64(*c),
            },
            (Some(Inst::Input(k)), _) => Op::Inp { d, idx: *k },
            (_, Some(s)) => Op::Inp {
                d,
                idx: self.index(s)?,
            },
            _ => return Err(CodegenError::Lost(v)),
        };
        self.push(op, Origin::Reload(v));
        self.bind(v, d);
        Ok(d)
    }
}
