/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A type written as a path: a generic parameter in scope; an alias, replaced by what it
 * stands for; or a struct or enum. The labels an alias or struct writes are placed where
 * the path stands. An item that takes generic arguments is given all of them here.
 */

use alloc::format;

use super::cx::Sema;
use super::defs::{DefId, DefKind};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{GenArg, TyId, Types};
use crate::compiler::syntax::ast::Path;
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /** The type the path `p` names, with the labels it writes. */
    pub(super) fn lower_path(
        &mut self,
        m: DefId,
        p: &'a Path,
        path: &[u32],
        labels: &mut Labels,
    ) -> TyId {
        match self.generic_named(p) {
            Some(GenArg::Type(t)) => return t,
            Some(GenArg::Const(_)) => {
                self.generic_kind(p, "a type");
                return Types::ERROR;
            }
            None => {}
        }
        let Some(def) = self.type_def(m, p) else {
            return Types::ERROR;
        };
        let kind = self.defs.get(def).map(|d| d.kind);
        if !matches!(kind, Some(DefKind::Alias | DefKind::Struct | DefKind::Enum)) {
            if let Some(k) = kind {
                let what = format!("`{}` is {}, not a type", p.last_name(), k.a());
                let d = Diagnostic::error(Code::WRONG_KIND, what, p.span, "not a type");
                self.diags.push(d);
            }
            return Types::ERROR;
        }
        let given = p.segments.last().and_then(|s| s.generics.as_deref());
        let params = (self.params_of(def), p.last_name());
        let Some(args) = self.type_args(m, params, given.unwrap_or(&[]), p.span) else {
            return Types::ERROR;
        };
        let (ty, own) = match kind {
            Some(DefKind::Alias) => self.alias_ty(def, &args),
            _ => self.adt_ty(def, &args),
        };
        for (q, l) in own.0 {
            labels.0.push(([path, &q].concat(), l));
        }
        ty
    }
}
