/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Variants named through their enum (section 7.11): `E::V`, or `Self::V` inside an `impl`
 * block of an enum. A lone name never names a variant; it binds, or names a local.
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::shape::Shape;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefKind;
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Path, PathRoot};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * If `p` goes through an enum: the enum and the variant's tag, or the enum alone if
     * its last name is no variant of it. `None` if `p` does not go through an enum.
     */
    pub(crate) fn variant_of(&mut self, p: &'a Path) -> Option<Result<Shape, TyId>> {
        let (last, prefix) = p.segments.split_last()?;
        let (ty, def) = match (p.root, prefix.is_empty()) {
            (PathRoot::SelfType, true) => (self.sema.self_ty?, None),
            (_, false) => {
                let names: Vec<&str> = prefix.iter().map(|s| s.ident.name.as_str()).collect();
                let def = self.sema.defs.resolve(self.module, p.root, &names).ok()?;
                if self.sema.defs.get(def).map(|d| d.kind) != Some(DefKind::Enum) {
                    return None;
                }
                (self.sema.struct_ty(def).0, Some(def))
            }
            _ => return None,
        };
        let adt = self.sema.types.adt(ty).filter(|a| a.is_enum)?;
        let tag = adt.variants.iter().position(|v| v.name == last.ident.name);
        if let Some(def) = def {
            self.sema.note_use(def, p);
        }
        Some(
            tag.and_then(|t| u32::try_from(t).ok())
                .map(|t| (ty, t))
                .ok_or(ty),
        )
    }

    /** The unit variant `p` names, as a value, if `p` goes through an enum. */
    pub(crate) fn variant_value(&mut self, p: &'a Path, at: Span) -> Option<TExpr> {
        match self.variant_of(p)? {
            Ok(_) => Some(self.unit_struct(p, at)),
            Err(ty) => Some(self.no_variant(p, ty).unwrap_or_else(|| self.error(at))),
        }
    }

    /** Report the last name of `p`, which is no variant of the enum `ty`. */
    pub(crate) fn no_variant<T>(&mut self, p: &'a Path, ty: TyId) -> Option<T> {
        let what = format!("`{}` has no variant `{}`", self.show(ty), p.last_name());
        let d = Diagnostic::error(Code::NO_FIELD, what, p.span, "no such variant");
        self.sema.diags.push(d);
        None
    }
}
