/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Replaying the three instructions that constrain a spilled copy. */

use super::verify_op::Replay;
use crate::compiler::ssa::{Hint, V};
use crate::isa::Op;

impl<'a> Replay<'a> {
    /** One of the three instructions that check a copy of `v`. */
    pub(super) fn spill_step(&mut self, op: Op, v: V) -> Result<(), &'static str> {
        let bad = "a spill that does not constrain its copy";
        self.spill = match (self.spill, op) {
            (None, Op::Inp { d, idx }) => {
                let s = self.slot(idx).ok_or(bad)?;
                (self.m.advice.get(s) == Some(&Hint::Copy(v)))
                    .then_some(())
                    .ok_or(bad)?;
                self.set(d, None);
                Some((v, s, d, 1))
            }
            (Some((w, s, r, 1)), Op::Sub { d, a, b }) if w == v && a == r && self.holds(b, v) => {
                self.set(d, None);
                Some((v, s, d, 2))
            }
            (Some((w, s, r, 2)), Op::Assert { a }) if w == v && a == r => {
                if let Some(c) = self.copies.get_mut(s) {
                    *c = true;
                }
                None
            }
            _ => return Err(bad),
        };
        Ok(())
    }
}
