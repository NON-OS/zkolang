/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What a literal or pattern of the struct forms builds or takes: a struct, which has one
 * shape, or one variant of an enum. Every field of a variant is visible where its enum is.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::field_of::Key;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{Form, TyId};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Path;

/** A struct's type, or an enum's and the tag of one of its variants; a struct's tag is 0. */
pub(crate) type Shape = (TyId, u32);

impl<'s, 'a> FnCx<'s, 'a> {
    /** The struct or variant `p` names, built in the form `form`; `None` once reported. */
    pub(crate) fn shape_named(&mut self, p: &'a Path, form: Form) -> Option<Shape> {
        match self.variant_of(p) {
            None => self.struct_named(p, form).map(|t| (t, 0)),
            Some(found) => self.variant_shape(p, found, form),
        }
    }

    /** The name and type of each field of the shape `s`. */
    pub(crate) fn shape_fields(&self, (ty, tag): Shape) -> Vec<(Option<String>, TyId)> {
        let v = self
            .sema
            .types
            .adt(ty)
            .and_then(|a| a.variants.get(tag as usize));
        v.map(|v| v.fields.iter().map(|f| (f.name.clone(), f.ty)).collect())
            .unwrap_or_default()
    }

    /** The index and type of the field `name` of the shape `s`; `None` once reported. */
    pub(super) fn shape_field(&mut self, s: Shape, name: &str, at: Span) -> Option<(u32, TyId)> {
        if !self.sema.types.adt(s.0).is_some_and(|a| a.is_enum) {
            return self.field_of(s.0, Key::Name(name), at);
        }
        let fields = self.shape_fields(s);
        let found = fields.iter().position(|f| f.0.as_deref() == Some(name));
        if let Some((i, f)) = found.and_then(|i| Some((i, fields.get(i)?))) {
            return u32::try_from(i).ok().map(|i| (i, f.1));
        }
        let what = format!("the variant `{}` has no field `{name}`", self.shape_name(s));
        let d = Diagnostic::error(Code::NO_FIELD, what, at, "no such field");
        self.sema.diags.push(d);
        None
    }
}
