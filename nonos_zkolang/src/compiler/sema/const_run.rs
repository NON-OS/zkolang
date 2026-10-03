/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Running a checked constant expression with the reference interpreter, within a budget
 * of ten million steps (section 11). What it uses is made ready first: the constants it
 * names are evaluated and the bodies of the functions it calls are checked.
 */

use super::cx::Sema;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::interp::{FailKind, Interp, Value};
use crate::compiler::tir::TExpr;

/** The steps a constant may take (section 11). */
pub(crate) const BUDGET: u64 = 10_000_000;

impl<'a> Sema<'a> {
    /** Run the checked constant expression `e`, in a frame of `locals` slots. */
    pub(crate) fn const_eval(&mut self, e: &TExpr, locals: usize) -> Option<Value> {
        if !self.prepare(e) {
            return None;
        }
        let out = Interp::new(&*self, BUDGET).eval_const(e, locals);
        match out {
            Ok(v) => Some(v),
            Err(f) => {
                let code = if f.kind == FailKind::Budget {
                    Code::CONST_BUDGET
                } else {
                    Code::CONST_EVAL_FAILED
                };
                let d = Diagnostic::error(
                    code,
                    alloc::format!("evaluating a constant failed: {}", f.kind.describe()),
                    f.span,
                    "fails here",
                );
                self.diags.push(d);
                None
            }
        }
    }

    /**
     * Run `e` as `const_eval` does, a failing run giving `None` unreported: a condition
     * that fails here is left to the run, where it fails only if reached (section 14).
     */
    pub(crate) fn try_const_eval(&mut self, e: &TExpr, locals: usize) -> Option<Value> {
        if !self.prepare(e) {
            return None;
        }
        Interp::new(&*self, BUDGET).eval_const(e, locals).ok()
    }
}
