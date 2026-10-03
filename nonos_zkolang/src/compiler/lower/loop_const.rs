/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The constant bound of a range loop, evaluated by the reference interpreter. */

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::interp::Interp;
use crate::compiler::tir::TExpr;

impl<'p> Lower<'p> {
    /** The value of the constant expression `e`. */
    pub(super) fn constant(&self, e: &TExpr) -> L<i128> {
        let v = Interp::new(self.p, 1_000_000).eval_const(e, 0);
        v.map(|v| v.int())
            .map_err(|_| LowerError::Unsupported("a loop bound that does not evaluate", e.span))
    }
}
