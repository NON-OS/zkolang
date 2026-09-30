/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Loops (sections 8.6 and 8.7): `for` over a range or an array, and `while ... limit`. */

use super::machine::{fail, Eval, Interp};
use super::{FailKind, Value};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'e> Interp<'e> {
    /** Run the loop `e`. */
    pub(super) fn eval_loop(&mut self, e: &TExpr) -> Eval {
        match &e.kind {
            TExprKind::ForRange {
                var,
                lo,
                hi,
                inclusive,
                body,
            } => {
                let (lo, hi) = (self.eval(lo)?.int(), self.eval(hi)?.int());
                let end = if *inclusive { hi.saturating_add(1) } else { hi };
                let mut v = lo;
                while v < end {
                    self.set(var.0, Value::Int(v));
                    if self.iteration(body)? {
                        break;
                    }
                    v += 1;
                }
            }
            TExprKind::ForArray {
                index,
                pat,
                array,
                body,
            } => {
                let items = self.eval(array)?;
                for (i, item) in items.parts().iter().enumerate() {
                    if let Some(ix) = index {
                        self.set(ix.0, Value::Int(i as i128));
                    }
                    self.bind(pat, item.clone());
                    if self.iteration(body)? {
                        break;
                    }
                }
            }
            TExprKind::While { cond, limit, body } => self.eval_while(cond, *limit, body)?,
            _ => return Err(fail(FailKind::Internal, e.span)),
        }
        Ok(Value::Unit)
    }
}
