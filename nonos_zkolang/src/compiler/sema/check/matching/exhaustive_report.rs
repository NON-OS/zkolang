/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reporting what the check of a `match` found: arms no value reaches, and a value none covers. */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::exhaust::check_match;
use crate::compiler::tir::{TArm, TExpr, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report the arms of `match s { arms }` that no value reaches, and a value none covers. */
    pub(super) fn exhaustive(&mut self, s: &TExpr, arms: &[TArm]) {
        let list: Vec<(&TPat, bool)> = arms.iter().map(|a| (&a.pat, a.guard.is_some())).collect();
        let Some(v) = check_match(&self.sema.types, s.ty, &list) else {
            let what = "this `match` has too many cases to check that it covers every value";
            let d = Diagnostic::error(Code::NON_EXHAUSTIVE, what, s.span, "too many cases")
                .with_help("split it into nested `match`es");
            self.sema.diags.push(d);
            return;
        };
        for i in v.unreachable {
            if let Some(a) = arms.get(i) {
                let d = Diagnostic::warning(
                    Code::UNREACHABLE_PATTERN,
                    "unreachable arm",
                    a.span,
                    "the arms before it cover every value it matches",
                );
                self.sema.diags.push(d);
            }
        }
        if let Some(m) = v.missing {
            let what = format!("the `match` does not cover `{m}`");
            let d = Diagnostic::error(
                Code::NON_EXHAUSTIVE,
                what,
                s.span,
                format!("`{m}` not covered"),
            )
            .with_help(format!(
                "add an arm for `{m}`, or one for `_` to cover the rest"
            ));
            self.sema.diags.push(d);
        }
    }
}
