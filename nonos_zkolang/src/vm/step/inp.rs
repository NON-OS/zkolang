/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Read an input into a register.

use nonos_stark::field::Fp;

use super::super::{ProveError, Vm};
use crate::trace::{OpTag, Row};

impl Vm {
    pub(super) fn step_inp(
        &mut self,
        d: u8,
        idx: u16,
        inputs: &[Fp],
        row: &mut Row,
    ) -> Result<(), ProveError> {
        row.op = OpTag::Inp;
        let v = *inputs.get(idx as usize).ok_or(ProveError::BadInput(idx))?;
        row.imm = v;
        row.rd = v;
        self.wset(d, v)
    }
}
