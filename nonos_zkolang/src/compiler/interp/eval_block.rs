/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Blocks, statements, `if`, and binding a pattern's locals. */

use super::machine::{fail, Eval, Interp};
use super::{FailKind, Value};
use crate::compiler::tir::{TBlock, TExpr, TPat, TStmt};

impl<'e> Interp<'e> {
    /** Run the statements of `b` in order, then its tail. */
    pub(super) fn eval_block(&mut self, b: &TBlock) -> Eval {
        for s in &b.stmts {
            match s {
                TStmt::Let { pat, init } => {
                    let v = self.eval(init)?;
                    self.bind(pat, v);
                }
                TStmt::Assert { cond, message } => {
                    if !self.eval(cond)?.bool() {
                        return Err(fail(FailKind::Assert(message.clone()), cond.span));
                    }
                }
                TStmt::Expr(e) => {
                    self.eval(e)?;
                }
            }
        }
        match &b.tail {
            Some(t) => self.eval(t),
            None => Ok(Value::Unit),
        }
    }

    /** `if c1 { .. } else if c2 { .. } else { .. }`: the first block whose condition holds. */
    pub(super) fn eval_if(&mut self, branches: &[(TExpr, TBlock)], last: Option<&TBlock>) -> Eval {
        for (cond, block) in branches {
            if self.eval(cond)?.bool() {
                return self.eval_block(block);
            }
        }
        match last {
            Some(b) => self.eval_block(b),
            None => Ok(Value::Unit),
        }
    }

    /** Store `v` in the locals `pat` binds. */
    pub(super) fn bind(&mut self, pat: &TPat, v: Value) {
        match pat {
            TPat::Bind(l) => {
                if let Some(slot) = self.frame.get_mut(l.0 as usize) {
                    *slot = v;
                }
            }
            TPat::Wild => {}
            TPat::Tuple(ps) | TPat::Variant(_, ps) => {
                if let Value::Tuple(parts) | Value::Array(parts) | Value::Variant(_, parts) = v {
                    for (p, part) in ps.iter().zip(parts) {
                        self.bind(p, part);
                    }
                }
            }
            TPat::Or(alts) => {
                if let Some(a) = alts.iter().find(|a| super::eval_match::matches(a, &v)) {
                    self.bind(a, v);
                }
            }
            TPat::Lit(..) | TPat::Range(..) => {}
        }
    }
}
