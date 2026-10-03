/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Paths in expressions (section 4.4): a local first, then a generic parameter, then a
 * constant item. A path over a primitive type names its associated constants `MIN`, `MAX`
 * and `BITS`.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::sema::defs::DefKind;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Path, PathRoot};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The value the path `p` names. */
    pub(crate) fn path_expr(&mut self, p: &'a Path, at: Span) -> TExpr {
        if let Some(e) = self.named_value(p, at) {
            return e;
        }
        if p.segments.is_empty() && p.root == PathRoot::SelfType {
            return self.unit_struct(p, at);
        }
        if p.segments.is_empty() && p.root == PathRoot::SelfModule {
            return self.self_value(at);
        }
        if let Some(e) = self.variant_value(p, at) {
            return e;
        }
        if let Some(e) = self.prim_const(p, at) {
            return e;
        }
        self.sema.no_generics(p);
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let def = match self.sema.defs.resolve(self.module, p.root, &names) {
            Ok(d) => d,
            Err(e) => {
                let names = self.names.iter().filter(|(_, v)| !v.is_empty());
                let locals: Vec<&str> = names.map(|(n, _)| n.as_str()).collect();
                self.sema.report_path(self.module, p, e, &locals);
                return self.error(at);
            }
        };
        self.sema.note_use(def, p);
        let kind = self.sema.defs.get(def).map(|d| d.kind);
        match (kind, self.sema.const_of.get(&def).copied()) {
            (Some(DefKind::Const), Some(c)) => {
                let ty = self.sema.const_ty(c);
                TExpr {
                    kind: TExprKind::Const(c),
                    ty,
                    span: at,
                }
            }
            (Some(DefKind::Struct), _) => self.unit_struct(p, at),
            (Some(k), _) => {
                self.not_a_value(p, k, at);
                self.error(at)
            }
            _ => self.error(at),
        }
    }
}
