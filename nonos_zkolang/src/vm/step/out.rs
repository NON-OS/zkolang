/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Write a register to the output vector.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::super::{ProveError, Vm};
use crate::trace::{OpTag, Row};

impl Vm {
    pub(super) fn step_out(
        &mut self,
        a: u8,
        idx: u16,
        outputs: &mut Vec<Fp>,
        row: &mut Row,
    ) -> Result<(), ProveError> {
        row.op = OpTag::Out;
        let v = self.rget(a)?;
        row.ra = v;
        let i = idx as usize;
        if outputs.len() <= i {
            outputs.resize(i + 1, Fp::ZERO);
        }
        outputs[i] = v;
        Ok(())
    }
}
