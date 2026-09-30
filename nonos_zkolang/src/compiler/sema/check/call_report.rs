/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What a call reports: a callee that is not a function, and a wrong number of arguments. */

use alloc::format;
use alloc::string::String;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::{DefId, DefKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report a call of `name`, which takes `want` arguments, with `got` (E0307). */
    pub(super) fn arity_of(&mut self, name: &str, want: usize, got: usize, at: Span) {
        if want != got {
            let what = format!("`{name}` takes {want} arguments, not {got}");
            let d = Diagnostic::error(Code::WRONG_ARITY, what, at, "wrong number of arguments");
            self.sema.diags.push(d);
        }
    }

    /** Why `def`, named `name`, cannot be called; empty if that is reported already. */
    pub(super) fn not_fn_message(&self, def: DefId, name: &str) -> String {
        match self.sema.defs.get(def).map_or(DefKind::Fn, |d| d.kind) {
            DefKind::Fn => String::new(),
            kind => format!("`{name}` is a {}, not a function", kind.describe()),
        }
    }

    /** Check `args` for their own errors, and report that `f` cannot be called. */
    pub(super) fn not_callable(
        &mut self,
        f: &'a Expr,
        args: &'a [Expr],
        message: &str,
        at: Span,
    ) -> TExpr {
        if !message.is_empty() {
            self.sema.diags.push(Diagnostic::error(
                Code::WRONG_KIND,
                String::from(message),
                f.span,
                "not a function",
            ));
        }
        for a in args {
            self.infer(a, None);
        }
        self.error(at)
    }
}
