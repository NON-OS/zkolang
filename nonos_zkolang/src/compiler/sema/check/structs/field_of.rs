/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Fields of tuples and structs (section 7.7): `t.0` of a tuple or tuple struct, `s.f` of a
 * struct with named fields. A struct's field is visible in its struct's module and the
 * modules inside it, and everywhere if `pub` (section 4.3).
 */

use alloc::format;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{Form, TyId, TyKind};
use crate::compiler::source::Span;

/** Which field: by position, or by name. */
#[derive(Clone, Copy)]
pub(crate) enum Key<'n> {
    Pos(u32),
    Name(&'n str),
}

impl<'s, 'a> FnCx<'s, 'a> {
    /** The index and type of field `key` of a value of type `ty`; `None` once reported. */
    pub(crate) fn field_of(&mut self, ty: TyId, key: Key<'_>, at: Span) -> Option<(u32, TyId)> {
        /* A variant's payload may still be a variable bound to the struct. */
        let ty = self.resolve(ty);
        let shown = match key {
            Key::Pos(i) => format!("{i}"),
            Key::Name(n) => alloc::string::String::from(n),
        };
        match (self.kind(ty), key) {
            (TyKind::Error, _) => return None,
            (TyKind::Tuple(ts), Key::Pos(i)) if (i as usize) < ts.len() => {
                return Some((i, ts[i as usize]));
            }
            _ => {}
        }
        let adt = self.sema.types.adt(ty).filter(|a| !a.is_enum).cloned();
        let found = adt.as_ref().and_then(|a| {
            let v = a.variants.first()?;
            let i = match (key, v.form) {
                (Key::Pos(i), Form::Tuple) => i,
                (Key::Name(n), Form::Named) => a.field(n)?.0,
                _ => return None,
            };
            Some((i, v.fields.get(i as usize)?.clone(), a.module))
        });
        let Some((i, f, module)) = found else {
            self.no_field(ty, &shown, at);
            return None;
        };
        if !f.public && !self.sema.defs.within(self.module, DefId(module)) {
            let what = format!("the field `{shown}` of `{}` is private", self.show(ty));
            let d = Diagnostic::error(Code::PRIVATE_ITEM, what, at, "private field");
            self.sema
                .diags
                .push(d.with_help("mark the field `pub` to use it outside its module"));
        }
        Some((i, f.ty))
    }
}
