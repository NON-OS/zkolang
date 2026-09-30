/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Calls (section 7.10): a named function with arguments of its parameter types, left to
 * right; a `&mut` parameter takes `&mut place` (section 10.3). A path over a primitive type
 * calls one of its associated functions.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use super::prim::prim_path;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::tir::{TArg, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `f(args)`. */
    pub(crate) fn call(&mut self, f: &'a Expr, args: &'a [Expr], at: Span) -> TExpr {
        let ExprKind::Path(p) = &f.kind else {
            return self.not_callable(f, args, "only a named function can be called", at);
        };
        if let Some((ty, name)) = prim_path(p) {
            return self.assoc_call(ty, name, args, at);
        }
        if p.as_ident().is_some_and(|i| self.lookup(&i.name).is_some()) {
            return self.not_callable(f, args, "a variable is not a function", at);
        }
        self.sema.no_generics(p);
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let def = match self.sema.defs.resolve(self.module, p.root, &names) {
            Ok(d) => d,
            Err(e) => {
                self.sema.report_path(p, e);
                return self.not_callable(f, args, "", at);
            }
        };
        self.sema.note_use(def, p);
        let Some(fid) = self.sema.fn_of.get(&def).copied() else {
            let message = self.not_fn_message(def, p.last_name());
            return self.not_callable(f, args, &message, at);
        };
        let sig = self.sema.sig(fid);
        self.arity_of(p.last_name(), sig.params.len(), args.len(), at);
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
