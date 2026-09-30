/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Small pieces the checks of calls and methods share. */

use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Ident};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** The type `[bool; n]`. */
    pub(crate) fn bits_ty(&mut self, n: u32) -> TyId {
        self.sema.types.intern(TyKind::Array(Types::BOOL, n))
    }

    /** Report a call of `method` with other than `n` arguments (E0307). */
    pub(crate) fn arity(&mut self, method: &Ident, n: usize, args: &[Expr], at: Span) {
        if args.len() != n {
            let d = Diagnostic::error(
                Code::WRONG_ARITY,
                format!("`{}` takes {n} arguments, not {}", method.name, args.len()),
                at,
                "wrong number of arguments",
            );
            self.sema.diags.push(d);
        }
    }

    /** Check `args` for their own errors, then stand for a call that failed. */
    pub(crate) fn check_args_then_error(&mut self, args: &'a [Expr], at: Span) -> TExpr {
        for a in args {
            self.infer(a, None);
        }
        self.error(at)
    }
}
