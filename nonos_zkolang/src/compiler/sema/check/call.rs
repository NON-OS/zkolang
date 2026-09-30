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
use crate::compiler::syntax::ast::{Expr, ExprKind, PathRoot};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `f(args)`. */
    pub(crate) fn call(&mut self, f: &'a Expr, args: &'a [Expr], at: Span) -> TExpr {
        let ExprKind::Path(p) = &f.kind else {
            return self.not_callable(f, args, "only a named function can be called", at);
        };
        if let Some((ty, name)) = prim_path(p) {
            return self.assoc_call(ty, name, args, at);
        }
        if p.root == PathRoot::SelfType && p.segments.is_empty() {
            return self.tuple_struct(p, args, at);
        }
        if let Some(found) = self.assoc_fn(p) {
            return match found {
                Some(fid) => self.call_fn(fid, p.last_name(), args, at),
                None => self.not_callable(f, args, "", at),
            };
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
        if self.sema.defs.get(def).map(|d| d.kind)
            == Some(crate::compiler::sema::defs::DefKind::Struct)
        {
            return self.tuple_struct(p, args, at);
        }
        self.sema.note_use(def, p);
        let Some(fid) = self.sema.fn_of.get(&def).copied() else {
            let message = self.not_fn_message(def, p.last_name());
            return self.not_callable(f, args, &message, at);
        };
        self.call_fn(fid, p.last_name(), args, at)
    }
}
