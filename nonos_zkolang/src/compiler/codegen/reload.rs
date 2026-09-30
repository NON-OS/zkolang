/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Bringing a value into a register: it is there; or it is a constant, an input or an
 * advice value read for the first time; or a constant or public input read again.
 */

use nonos_stark::field::Fp;

use super::machine::{CodegenError, Origin};
use super::state::Cg;
use crate::compiler::ssa::{Inst, V};
use crate::isa::Op;

impl<'a> Cg<'a> {
    /** The register holding `v`, reading it if none does; `keep` stays in place. */
    pub(super) fn ensure(&mut self, v: V, keep: &[V]) -> Result<u8, CodegenError> {
        if let Some(Some(r)) = self.reg.get(v.index()) {
            return Ok(*r);
        }
        let again = self.emitted.get(v.index()).copied().unwrap_or(true);
        if again && !self.recoverable(v) {
            return Err(CodegenError::Lost(v));
        }
        let d = self.take_reg(keep)?;
        let op = match (self.ssa.get(v), self.slot.get(v.index()).copied().flatten()) {
            (Some(Inst::Const(c)), _) => Op::Imm {
                d,
                v: Fp::from_u64(*c),
            },
            (Some(Inst::Input(k)), _) => Op::Inp { d, idx: *k },
            (Some(Inst::Advice(_)), Some(s)) => Op::Inp {
                d,
                idx: self.index(s)?,
            },
            _ => return Err(CodegenError::Lost(v)),
        };
        self.push(
            op,
            if again {
                Origin::Reload(v)
            } else {
                Origin::Def(v)
            },
        );
        if let Some(e) = self.emitted.get_mut(v.index()) {
            *e = true;
        }
        self.bind(v, d);
        Ok(d)
    }

    /** The input index of advice slot `s`. */
    pub(super) fn index(&self, s: u16) -> Result<u16, CodegenError> {
        let i = self.out.n_inputs.checked_add(usize::from(s));
        i.and_then(|i| u16::try_from(i).ok())
            .ok_or(CodegenError::TooManySlots)
    }

    /** Append an instruction and where it comes from. */
    pub(super) fn push(&mut self, op: Op, o: Origin) {
        self.out.ops.push(op);
        self.out.origins.push(o);
    }

    /** The next position `v` is read at, if any. */
    pub(super) fn next_use(&self, v: V) -> Option<u32> {
        let (u, p) = (self.uses.get(v.index())?, *self.passed.get(v.index())?);
        u.get(p).copied()
    }
}
