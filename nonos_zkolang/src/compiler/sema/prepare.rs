/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Making ready what a constant expression uses: every constant it names evaluated, and
 * every function it calls checked, with what that function uses, in turn. A body met
 * while it is still being checked is part of a cycle (E0502).
 */

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use super::cx::{Sema, State};
use super::uses::collect;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::tir::{FnId, TExpr, TExprKind};

impl<'a> Sema<'a> {
    /** Make ready what `e` uses; false if any of it fails, which is reported. */
    pub(crate) fn prepare(&mut self, e: &TExpr) -> bool {
        let mut seen = BTreeSet::new();
        let mut todo: Vec<TExpr> = alloc::vec![e.clone()];
        while let Some(e) = todo.pop() {
            let mut uses = (Vec::new(), Vec::new());
            collect(&e, &mut uses);
            for c in uses.0 {
                if self.const_value(c).is_none() {
                    return false;
                }
            }
            for f in uses.1 {
                if !seen.insert(f) {
                    continue;
                }
                if !self.body_ready(f, e.span) {
                    return false;
                }
                /* A body with errors is reported already; running it would only fail. */
                if !self.fns.get(f.0 as usize).is_some_and(|i| i.clean) {
                    return false;
                }
                if let Some(State::Done(body)) = self.fns.get(f.0 as usize).map(|i| &i.body) {
                    todo.push(TExpr {
                        kind: TExprKind::Block(body.body.clone()),
                        ty: body.ret,
                        span: body.span,
                    });
                }
            }
        }
        true
    }

    /** Check the body of `f`, needed at `at`; false if it fails or is part of a cycle. */
    fn body_ready(&mut self, f: FnId, at: Span) -> bool {
        if matches!(
            self.fns.get(f.0 as usize).map(|i| &i.body),
            Some(State::Checking)
        ) {
            let d = Diagnostic::error(
                Code::CONST_CYCLE,
                "evaluating this needs the function it is in",
                at,
                "the function is still being checked",
            );
            self.diags.push(d);
            return false;
        }
        self.check_body(f)
    }
}
