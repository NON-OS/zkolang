/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A path of one name where a value stands: a local, else a generic parameter (section
 * 11). A constant parameter is the `usize` its instance gives it, a constant expression;
 * a type parameter is no value.
 */

use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{GenArg, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Path;
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::{TExpr, TExprKind, TLit};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The local, else the generic parameter, a path of one name names, if it names one. */
    pub(super) fn named_value(&mut self, p: &Path, at: Span) -> Option<TExpr> {
        if let Some(l) = p.as_ident().and_then(|id| self.lookup(&id.name)) {
            self.read_local(l);
            let ty = self.locals.get(l.0 as usize).map_or(Types::ERROR, |x| x.ty);
            return Some(TExpr {
                kind: TExprKind::Local(l),
                ty,
                span: at,
            });
        }
        match self.sema.generic_named(p)? {
            GenArg::Const(n) => Some(TExpr {
                kind: TExprKind::Lit(TLit::Int(i128::from(n))),
                ty: Types::int(IntTy::Usize),
                span: at,
            }),
            GenArg::Type(_) => {
                let what = format!("`{}` is a type parameter, not a value", p.last_name());
                let d = Diagnostic::error(Code::WRONG_KIND, what, at, "not a value");
                self.sema.diags.push(d);
                Some(self.error(at))
            }
        }
    }
}
