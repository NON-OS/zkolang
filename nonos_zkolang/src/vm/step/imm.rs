/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Load an immediate.

use nonos_stark::field::Fp;

use super::super::{ProveError, Vm};
use crate::trace::{OpTag, Row};

impl Vm {
    pub(super) fn step_imm(&mut self, d: u8, v: Fp, row: &mut Row) -> Result<(), ProveError> {
        row.op = OpTag::Imm;
        row.imm = v;
        row.rd = v;
        self.wset(d, v)
    }
}
