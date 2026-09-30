/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `value.m(args)` for a method of an `impl` block (section 10.2): the receiver is the
 * first argument, a value for `self` and a place for `&mut self`.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{GenArg, TyId};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Ident, Param};
use crate::compiler::tir::{FnId, TArg, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The call of the method `fid` on `receiver`, of type `ty`, checked as `recv` with the
     * diagnostics from `mark` on, which a `&mut self` receiver checks again as a place;
     * for a generic method, `generics` are its template's arguments.
     */
    pub(super) fn user_method(
        &mut self,
        (fid, generics): (FnId, Option<Vec<GenArg>>),
        (receiver, recv, ty, mark): (&'a Expr, TExpr, TyId, usize),
        method: &Ident,
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        let is_method = self
            .sema
            .fns
            .get(fid.0 as usize)
            .and_then(|f| f.decl.params.first());
        if !matches!(is_method, Some(Param::SelfParam { .. })) {
            return self.not_a_method(ty, method, args, at);
        }
        let sig = match &generics {
            Some(g) => self.sema.sig_with(fid, g),
            None => self.sema.sig(fid),
        };
        let first = match sig.params.first().is_some_and(|p| p.2) {
            true => {
                self.sema.diags.rewind(mark);
                let p = self.place(receiver, Some(ty));
                self.read_local(p.root);
                TArg::Place(p)
            }
            false => TArg::Value(recv),
        };
        self.arity_of(
            &method.name,
            sig.params.len().saturating_sub(1),
            args.len(),
            at,
        );
        let mut targs = Vec::with_capacity(args.len() + 1);
        targs.push(first);
        for (i, a) in args.iter().enumerate() {
            targs.push(self.arg(a, sig.params.get(i + 1)));
        }
        self.check_disjoint(&targs);
        if let Some(g) = generics {
            self.pending.push((at, g));
        }
        TExpr {
            kind: TExprKind::Call(fid, targs),
            ty: sig.ret,
            span: at,
        }
    }
}
