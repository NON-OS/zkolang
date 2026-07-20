/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The boolean constraint.

use super::super::{ProveError, Vm};
use super::is_bool::is_bool;
use crate::trace::{OpTag, Row};

impl Vm {
    pub(super) fn step_bool(&mut self, a: u8, row: &mut Row, clk: u64) -> Result<(), ProveError> {
        row.op = OpTag::Bool;
        let va = self.rget(a)?;
        row.ra = va;
        row.aux = va;
        if !is_bool(va) && self.check {
            return Err(ProveError::Unprovable { step: clk });
        }
        Ok(())
    }
}
