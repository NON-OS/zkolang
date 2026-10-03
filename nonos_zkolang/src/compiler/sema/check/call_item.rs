/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A call of an item a path names: a tuple struct is built; a function is called, a
 * generic one through its template, and generic arguments written on any other refused.
 */

use super::cx::FnCx;
use crate::compiler::sema::defs::{DefId, DefKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Path};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `f(args)`, where `f` is the path `p`, which names the item `def`. */
    pub(super) fn call_item(
        &mut self,
        f: &'a Expr,
        (p, def): (&'a Path, DefId),
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        if self.sema.defs.get(def).map(|d| d.kind) == Some(DefKind::Struct) {
            return self.tuple_struct(p, args, at);
        }
        self.sema.note_use(def, p);
        let Some(fid) = self.sema.fn_of.get(&def).copied() else {
            self.sema.no_generics(p);
            let message = self.not_fn_message(def, p.last_name());
            return self.not_callable(f, args, &message, at);
        };
        if self
            .sema
            .fns
            .get(fid.0 as usize)
            .is_some_and(|i| i.template)
        {
            return self.call_generic(fid, p, args, at);
        }
        self.sema.no_generics(p);
        self.call_fn(fid, p.last_name(), args, at)
    }
}
