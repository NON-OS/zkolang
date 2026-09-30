/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The arguments checked before a generic function's instance was known, fitted to its
 * parameter types; and a call whose arguments give not every constant (E0303).
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{Labels, TArg, TExpr};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Check that `arg`, checked on its own, fits the parameter `param`. */
    pub(super) fn fit_arg(&mut self, arg: &TArg, param: Option<&(TyId, Labels, bool)>) {
        let Some(&(want, _, _)) = param else {
            return;
        };
        match arg {
            TArg::Value(e) => self.coerce(e, want),
            TArg::Place(p) => {
                if !self.unify(p.ty, want) {
                    self.mismatch(p.span, want, p.ty);
                }
            }
        }
    }

    /** Report the constant `c` of `name`, which no argument gives, and check the rest. */
    pub(super) fn not_inferred(
        &mut self,
        (c, name): (&str, &str),
        args: &'a [Expr],
        first: Vec<Option<TArg>>,
        at: Span,
    ) -> TExpr {
        let what = format!("cannot infer the constant `{c}` of `{name}`");
        let d = Diagnostic::error(Code::CANNOT_INFER, what, at, "no argument gives it")
            .with_help(format!("write it: `{name}::<..>`"));
        self.sema.diags.push(d);
        for (i, a) in args.iter().enumerate() {
            if !matches!(first.get(i), Some(Some(_))) {
                self.infer(a, None);
            }
        }
        self.error(at)
    }
}
