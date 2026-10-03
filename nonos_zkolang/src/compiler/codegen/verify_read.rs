/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Replaying a read of an input, advice slot or constant. */

use super::verify_op::Replay;
use crate::compiler::ssa::V;
use crate::isa::Op;

impl<'a> Replay<'a> {
    /**
     * A read of `v`. A row reading a secret input or an advice slot is pinned to nothing,
     * so a second read of one could differ from the first: each is read at most once.
     */
    pub(super) fn reload(&mut self, op: Op, v: V) -> Result<(), &'static str> {
        let d = self
            .reads(op, v)
            .ok_or("a read that does not read the value")?;
        if let Op::Inp { idx, .. } = op {
            let i = usize::from(idx);
            if i >= usize::from(self.ssa.n_public) {
                let seen = self.read.get_mut(i).ok_or("a read past the advice")?;
                if core::mem::replace(seen, true) {
                    return Err("a secret input or advice slot read twice");
                }
            }
        }
        self.set(d, Some(v));
        Ok(())
    }
}
