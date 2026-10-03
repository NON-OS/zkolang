/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Variants named through their enum (section 7.11): `E::V`, or `Self::V` inside an `impl`
 * block of an enum. Generic arguments written after the path, `E::V::<A>`, are the
 * enum's. A lone name names a variant only of the prelude's enums (section 18.1). The
 * enum is instantiated only once the variant is found.
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::shape::Shape;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefId;
use crate::compiler::syntax::ast::{ItemKind, Path, PathRoot};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * If `p` goes through an enum: its instance and the variant's tag, or the enum alone
     * if its last name is no variant of it. `None` if `p` does not go through an enum.
     */
    pub(crate) fn variant_of(&mut self, p: &'a Path) -> Option<Result<Shape, DefId>> {
        let (last, prefix) = p.segments.split_last()?;
        if p.root == PathRoot::SelfType && prefix.is_empty() {
            let ty = self.sema.self_ty?;
            let adt = self.sema.types.adt(ty).filter(|a| a.is_enum)?;
            let tag = adt.variants.iter().position(|v| v.name == last.ident.name);
            let tag = tag.and_then(|t| u32::try_from(t).ok());
            return Some(tag.map(|t| (ty, t)).ok_or(DefId(adt.def)));
        }
        if prefix.is_empty() && p.root == PathRoot::Plain {
            return self.prelude_path(p);
        }
        if prefix.is_empty() || p.root == PathRoot::SelfType {
            return None;
        }
        let names: Vec<&str> = prefix.iter().map(|s| s.ident.name.as_str()).collect();
        let def = self.sema.defs.resolve(self.module, p.root, &names).ok()?;
        let Some(ItemKind::Enum(e)) = self
            .sema
            .defs
            .get(def)
            .and_then(|d| d.item)
            .map(|i| &i.kind)
        else {
            return None;
        };
        self.sema.note_use(def, p);
        let tag = e
            .variants
            .iter()
            .position(|v| v.name.name == last.ident.name);
        let Some(tag) = tag.and_then(|t| u32::try_from(t).ok()) else {
            return Some(Err(def));
        };
        Some(Ok((
            self.instance(def, last.generics.as_deref(), p.span),
            tag,
        )))
    }

    /** Report the last name of `p`, which is no variant of the enum `def`. */
    pub(crate) fn no_variant<T>(&mut self, p: &'a Path, def: DefId) -> Option<T> {
        let name = self.sema.defs.get(def).map_or("", |d| d.name.as_str());
        let what = format!("`{name}` has no variant `{}`", p.last_name());
        let d = Diagnostic::error(Code::NO_FIELD, what, p.span, "no such variant");
        self.sema.diags.push(d);
        None
    }
}
