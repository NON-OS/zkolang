/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The witnessed field inverse.

use nonos_stark::field::Fp;

use super::super::{ProveError, Vm};
use crate::trace::{OpTag, Row};

impl Vm {
    pub(super) fn step_inv(
        &mut self,
        d: u8,
        a: u8,
        row: &mut Row,
        clk: u64,
    ) -> Result<(), ProveError> {
        row.op = OpTag::Inv;
        let va = self.rget(a)?;
        row.ra = va;
        if va == Fp::ZERO {
            if self.check {
                return Err(ProveError::Unprovable { step: clk });
            }
            row.rd = Fp::ZERO;
            row.aux = Fp::ZERO;
            return self.wset(d, Fp::ZERO);
        }
        let inv = va.inv();
        row.rd = inv;
        row.aux = inv;
        self.wset(d, inv)
    }
}
