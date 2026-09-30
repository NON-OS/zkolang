/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `while ... limit` (section 8.7), and the steps every loop shares. */

use super::machine::{fail, Flow, Interp};
use super::{FailKind, Value};
use crate::compiler::tir::{TBlock, TExpr};

impl<'e> Interp<'e> {
    /** `while cond limit n { body }`: fails when `cond` still holds after `n` iterations. */
    pub(super) fn eval_while(
        &mut self,
        cond: &TExpr,
        limit: u32,
        body: &TBlock,
    ) -> Result<(), Flow> {
        let mut n = 0u32;
        while self.eval(cond)?.bool() {
            if n == limit {
                return Err(fail(FailKind::LimitExceeded, cond.span));
            }
            n += 1;
            if self.iteration(body)? {
                break;
            }
        }
        Ok(())
    }

    /** Run one iteration of `body`; say whether a `break` ended the loop. */
    pub(super) fn iteration(&mut self, body: &TBlock) -> Result<bool, Flow> {
        match self.eval_block(body) {
            Ok(_) | Err(Flow::Continue) => Ok(false),
            Err(Flow::Break) => Ok(true),
            Err(other) => Err(other),
        }
    }

    /** Set local `l`. */
    pub(super) fn set(&mut self, l: u32, v: Value) {
        if let Some(slot) = self.frame.get_mut(l as usize) {
            *slot = v;
        }
    }
}
