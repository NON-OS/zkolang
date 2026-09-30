/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `const fn` (section 11): its body uses no `declassify` and no labelled type, and calls
 * only other `const fn`s (E0504).
 */

use alloc::vec::Vec;

use super::cx::{Sema, State};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'a> Sema<'a> {
    /** Check every `const fn` whose body checked. */
    pub(crate) fn check_const_fns(&mut self) {
        let mut found: Vec<(Span, &'static str)> = Vec::new();
        for info in &self.fns {
            let State::Done(body) = &info.body else {
                continue;
            };
            if !body.is_const {
                continue;
            }
            let labelled = body
                .locals
                .iter()
                .find(|l| !l.labels.is_empty())
                .map(|l| l.span);
            if let Some(at) = labelled.or(Some(body.span).filter(|_| !body.ret_labels.is_empty())) {
                found.push((at, "a `const fn` uses no `secret` or `public` type"));
            }
            body.body
                .each_expr(&mut |e| self.const_fn_uses(e, &mut found));
        }
        for (at, message) in found {
            self.diags.push(Diagnostic::error(
                Code::NOT_CONST_FN,
                message,
                at,
                "not allowed in a `const fn`",
            ));
        }
    }

    fn const_fn_uses(&self, e: &TExpr, found: &mut Vec<(Span, &'static str)>) {
        match &e.kind {
            TExprKind::Declassify(_) => found.push((e.span, "a `const fn` uses no `declassify`")),
            TExprKind::Call(f, _)
                if !self.fns.get(f.0 as usize).is_some_and(|i| i.decl.is_const) =>
            {
                found.push((e.span, "a `const fn` calls only `const fn`s"))
            }
            _ => {}
        }
        e.each_child(&mut |c| self.const_fn_uses(c, found));
    }
}
