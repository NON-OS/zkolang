/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `return`, `break` and `continue` (section 8.8), which have type `!`: they leave, so
 * their value is never used.
 */

use alloc::boxed::Box;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::Types;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `return v`, which leaves the function with `v` or `()`. */
    pub(crate) fn return_expr(&mut self, v: Option<&'a Expr>, at: Span) -> TExpr {
        let Some(ret) = self.ret else {
            let d = Diagnostic::error(
                Code::MISPLACED_STATEMENT,
                "`return` stands only in a function",
                at,
                "not in a function",
            );
            self.sema.diags.push(d);
            return self.error(at);
        };
        let v = match v {
            Some(v) => Some(Box::new(self.expr(v, Some(ret)))),
            None => {
                if !self.unify(ret, Types::UNIT) {
                    self.mismatch(at, ret, Types::UNIT);
                }
                None
            }
        };
        TExpr {
            kind: TExprKind::Return(v),
            ty: Types::NEVER,
            span: at,
        }
    }

    /** `break` or `continue`, which stand only inside a loop (E0311). */
    pub(crate) fn loop_exit(&mut self, is_break: bool, at: Span) -> TExpr {
        if self.loops == 0 {
            let what = if is_break { "`break`" } else { "`continue`" };
            let d = Diagnostic::error(
                Code::OUTSIDE_LOOP,
                alloc::format!("{what} outside a loop"),
                at,
                "not inside a loop",
            );
            self.sema.diags.push(d);
            return self.error(at);
        }
        let kind = if is_break {
            TExprKind::Break
        } else {
            TExprKind::Continue
        };
        TExpr {
            kind,
            ty: Types::NEVER,
            span: at,
        }
    }
}
