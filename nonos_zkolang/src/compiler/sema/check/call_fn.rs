/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A call of a known function: its arity, its arguments at its parameters' types, their places apart. */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{FnId, TArg, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** A call of the function `fid`, named `name`, on `args`. */
    pub(crate) fn call_fn(&mut self, fid: FnId, name: &str, args: &'a [Expr], at: Span) -> TExpr {
        let sig = self.sema.sig(fid);
        self.arity_of(name, sig.params.len(), args.len(), at);
        let targs: Vec<TArg> = args
            .iter()
            .enumerate()
            .map(|(i, a)| self.arg(a, sig.params.get(i)))
            .collect();
        self.check_disjoint(&targs);
        TExpr {
            kind: TExprKind::Call(fid, targs),
            ty: sig.ret,
            span: at,
        }
    }
}
