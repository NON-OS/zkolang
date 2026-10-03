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
use crate::compiler::syntax::ast::{Expr, Ident};
use crate::compiler::tir::{FnId, TArg, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The call of the method `fid`, which takes `self`, on `receiver`, of type `ty`, checked
     * as `recv` with the diagnostics from `mark` on, which a `&mut self` receiver checks
     * again as a place; for a generic method, `generics` are its template's arguments and
     * `early` holds the arguments checked already to settle its constants.
     */
    pub(super) fn user_method(
        &mut self,
        (fid, generics, mut early): (FnId, Option<Vec<GenArg>>, Vec<Option<TArg>>),
        (receiver, recv, ty, mark): (&'a Expr, TExpr, TyId, usize),
        method: &Ident,
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
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
            let param = sig.params.get(i + 1);
            targs.push(self.arg_for(a, param, early.get_mut(i).and_then(Option::take)));
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
