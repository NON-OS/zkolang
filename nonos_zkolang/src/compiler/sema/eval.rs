/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Checking and evaluating a constant expression written in the program (section 11). */

use alloc::vec::Vec;

use super::check::FnCx;
use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::interp::Value;
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{TExpr, TLocal};

impl<'a> Sema<'a> {
    /** Check `e`, written in module `m`, as a constant expression of type `ty`. */
    pub(crate) fn check_const_expr(
        &mut self,
        m: DefId,
        e: &'a Expr,
        ty: TyId,
    ) -> Option<(TExpr, Vec<TLocal>)> {
        let errors = self.diags.error_count();
        let mut cx = FnCx::new(self, m, None);
        let mut t = cx.expr(e, Some(ty));
        cx.settle();
        cx.rewrite(&mut t);
        cx.post(&t);
        let locals = core::mem::take(&mut cx.locals);
        if self.diags.error_count() != errors {
            return None;
        }
        if let Some(at) = self.not_const(&t) {
            let d = Diagnostic::error(Code::NOT_CONSTANT, "this is not a constant expression", at, "not constant")
                .with_help("a constant expression uses literals, constants and `const fn` calls, not variables");
            self.diags.push(d);
            return None;
        }
        Some((t, locals))
    }

    /** The value of `e`, written in module `m`, a constant expression of type `ty`. */
    pub(crate) fn eval_expr(&mut self, m: DefId, e: &'a Expr, ty: TyId) -> Option<Value> {
        let (t, locals) = self.check_const_expr(m, e, ty)?;
        self.const_eval(&t, locals.len())
    }
}
