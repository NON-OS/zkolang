/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The variants of an enum declaration (section 5.2), in order: the order is the tag. A
 * variant's fields have the enum's visibility (section 4.3). A name declared twice is
 * reported (E0201).
 */

use alloc::format;
use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{AdtVariant, Form};
use crate::compiler::syntax::ast::{FieldDecl, Fields, Variant};

/** How `fields` are written, and their declarations. */
pub(super) fn form_of(fields: &Fields) -> (Form, &[FieldDecl]) {
    match fields {
        Fields::Unit => (Form::Unit, &[]),
        Fields::Tuple(f) => (Form::Tuple, f),
        Fields::Named(f) => (Form::Named, f),
    }
}

impl<'a> Sema<'a> {
    /** The variants `vs` of an enum declared in module `m`. */
    pub(super) fn variants(&mut self, m: DefId, vs: &'a [Variant]) -> Vec<AdtVariant> {
        let mut out: Vec<AdtVariant> = Vec::with_capacity(vs.len());
        for v in vs {
            if out.iter().any(|w| w.name == v.name.name) {
                let what = format!("the variant `{}` is declared twice", v.name.name);
                let d =
                    Diagnostic::error(Code::DUPLICATE_ITEM, what, v.name.span, "declared again");
                self.diags.push(d);
            }
            let (form, decls) = form_of(&v.fields);
            let (mut fields, _) = self.fields(m, decls);
            fields.iter_mut().for_each(|f| f.public = true);
            out.push(AdtVariant {
                name: v.name.name.clone(),
                form,
                fields,
            });
        }
        out
    }
}
