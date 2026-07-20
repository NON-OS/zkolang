/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The shared add, subtract, and multiply body.

use nonos_stark::field::Fp;

use super::super::{ProveError, Vm};
use crate::trace::{OpTag, Row};

impl Vm {
    /// Read two registers, record them, apply the field operation, write the result.
    pub(super) fn arith(
        &mut self,
        tag: OpTag,
        d: u8,
        a: u8,
        b: u8,
        row: &mut Row,
        f: fn(Fp, Fp) -> Fp,
    ) -> Result<(), ProveError> {
        row.op = tag;
        let va = self.rget(a)?;
        let vb = self.rget(b)?;
        row.ra = va;
        row.rb = vb;
        let out = f(va, vb);
        row.rd = out;
        self.wset(d, out)
    }
}
