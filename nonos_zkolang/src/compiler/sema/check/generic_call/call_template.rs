/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A call of a template for its generic arguments: the instance's signature for them, each
 * argument checked against its parameter, or fitted to it if checked before the
 * arguments were known. The call is settled on its instance when the body is.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{FnId, TArg, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `call_template`, the arguments in `first` checked already. */
    pub(super) fn call_template_with(
        &mut self,
        t: FnId,
        (name, generics): (&str, Vec<GenArg>),
        args: &'a [Expr],
        mut first: Vec<Option<TArg>>,
        at: Span,
    ) -> TExpr {
        let sig = self.sema.sig_with(t, &generics);
        self.arity_of(name, sig.params.len(), args.len(), at);
        let mut targs: Vec<TArg> = Vec::with_capacity(args.len());
        for (i, a) in args.iter().enumerate() {
            let param = sig.params.get(i);
            targs.push(match first.get_mut(i).and_then(Option::take) {
                Some(arg) => {
                    self.fit_arg(&arg, param);
                    arg
                }
                None => self.arg(a, param),
            });
        }
        self.check_disjoint(&targs);
        self.pending.push((at, generics));
        TExpr {
            kind: TExprKind::Call(t, targs),
            ty: sig.ret,
            span: at,
        }
    }
}
