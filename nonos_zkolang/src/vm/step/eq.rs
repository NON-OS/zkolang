/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The witnessed equality bit.

use nonos_stark::field::Fp;

use super::super::{ProveError, Vm};
use crate::trace::{OpTag, Row};

impl Vm {
    pub(super) fn step_eq(&mut self, d: u8, a: u8, b: u8, row: &mut Row) -> Result<(), ProveError> {
        row.op = OpTag::Eq;
        let va = self.rget(a)?;
        let vb = self.rget(b)?;
        row.ra = va;
        row.rb = vb;
        let diff = va - vb;
        let (eq, aux) = if diff == Fp::ZERO {
            (Fp::ONE, Fp::ZERO)
        } else {
            (Fp::ZERO, diff.inv())
        };
        row.rd = eq;
        row.aux = aux;
        self.wset(d, eq)
    }
}
