/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The variants of the prelude's enums named alone (section 18.1): `Some(x)` and `None`
 * stand for `Option::Some(x)` and `Option::None` unless an item of that name is in scope.
 * In a pattern, a lone name that names a unit variant so takes that variant.
 */

use super::super::cx::FnCx;
use super::shape::Shape;
use crate::compiler::sema::defs::DefId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Fields, GenericArg, ItemKind, Path, PathRoot};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The variant of a prelude enum the lone name `name` names, at `at`, with `given` if
     * written, and whether it has no fields; `None` if it names none.
     */
    pub(crate) fn prelude_variant(
        &mut self,
        name: &str,
        given: Option<&'a [GenericArg]>,
        at: Span,
    ) -> Option<(Shape, bool)> {
        let defs = &self.sema.defs;
        if defs.resolve(self.module, PathRoot::Plain, &[name]).is_ok() {
            return None;
        }
        let def: DefId = defs.prelude_variant(name)?;
        let Some(ItemKind::Enum(e)) = defs.get(def).and_then(|d| d.item).map(|i| &i.kind) else {
            return None;
        };
        let tag = e.variants.iter().position(|v| v.name.name == name)?;
        let unit = e
            .variants
            .get(tag)
            .is_some_and(|v| matches!(v.fields, Fields::Unit));
        let tag = u32::try_from(tag).ok()?;
        Some(((self.instance(def, given, at), tag), unit))
    }

    /** The variant of a prelude enum the lone path `p` names, if it names one. */
    pub(super) fn prelude_path(&mut self, p: &'a Path) -> Option<Result<Shape, DefId>> {
        let given = p.segments.last().and_then(|s| s.generics.as_deref());
        let (shape, _) = self.prelude_variant(p.last_name(), given, p.span)?;
        Some(Ok(shape))
    }
}
