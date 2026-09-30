/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The arguments checked before a generic function's instance is known: those whose
 * parameter's type names a constant parameter, each checked on its own, a `&mut`
 * parameter taking a place.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::const_names::{names_const, Consts};
use crate::compiler::sema::ty::Types;
use crate::compiler::syntax::ast::{Expr, ExprKind, Param, TypeKind};
use crate::compiler::tir::{FnId, Labels, TArg};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Check the arguments of `t(args)` that give constants, binding them in `c`. */
    pub(super) fn args_first(
        &mut self,
        t: FnId,
        args: &'a [Expr],
        mut c: Consts<'_>,
    ) -> Vec<Option<TArg>> {
        let Some((m, decl)) = self.sema.fns.get(t.0 as usize).map(|i| (i.module, i.decl)) else {
            return Vec::new();
        };
        let mut first = Vec::with_capacity(args.len());
        for (i, a) in args.iter().enumerate() {
            let ty = match decl.params.get(i) {
                Some(Param::Typed { ty, .. }) if names_const(ty, c.0) => ty,
                _ => {
                    first.push(None);
                    continue;
                }
            };
            let arg = self.arg_alone(a, matches!(ty.kind, TypeKind::RefMut(_)));
            let found = match &arg {
                TArg::Value(e) => e.ty,
                TArg::Place(p) => p.ty,
            };
            self.bind_consts(m, ty, found, &mut c);
            first.push(Some(arg));
        }
        first
    }

    /** The argument `a` checked on its own, for a parameter that takes a place if `by_ref`. */
    fn arg_alone(&mut self, a: &'a Expr, by_ref: bool) -> TArg {
        match (&a.kind, by_ref) {
            (ExprKind::RefMut(place), true) => {
                let p = self.place(place, None);
                self.read_local(p.root);
                TArg::Place(p)
            }
            (ExprKind::RefMut(_), false) | (_, true) => {
                self.arg(a, Some(&(Types::ERROR, Labels::default(), by_ref)))
            }
            (_, false) => TArg::Value(self.infer(a, None)),
        }
    }
}
