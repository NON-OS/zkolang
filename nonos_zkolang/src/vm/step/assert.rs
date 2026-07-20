/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The zero-assertion constraint.

use nonos_stark::field::Fp;

use super::super::{ProveError, Vm};
use crate::trace::{OpTag, Row};

impl Vm {
    pub(super) fn step_assert(&mut self, a: u8, row: &mut Row, clk: u64) -> Result<(), ProveError> {
        row.op = OpTag::Assert;
        let va = self.rget(a)?;
        row.ra = va;
        row.aux = va;
        if va != Fp::ZERO {
            return Err(ProveError::Unprovable { step: clk });
        }
        Ok(())
    }
}
