/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The branchless select.

use super::super::{ProveError, Vm};
use super::is_bool::is_bool;
use crate::trace::{OpTag, Row};

impl Vm {
    pub(super) fn step_sel(
        &mut self,
        d: u8,
        c: u8,
        a: u8,
        b: u8,
        row: &mut Row,
        clk: u64,
    ) -> Result<(), ProveError> {
        row.op = OpTag::Sel;
        let vc = self.rget(c)?;
        let va = self.rget(a)?;
        let vb = self.rget(b)?;
        row.rc = vc;
        row.ra = va;
        row.rb = vb;
        if !is_bool(vc) && self.check {
            return Err(ProveError::Unprovable { step: clk });
        }
        /*
         * The arms blend as the constraint states them. For a boolean condition that picks
         * one arm; an unchecked evaluation, as the advice fill runs before its bits settle,
         * may meet any condition and must carry on.
         */
        let out = vc * (va - vb) + vb;
        row.rd = out;
        self.wset(d, out)
    }
}
