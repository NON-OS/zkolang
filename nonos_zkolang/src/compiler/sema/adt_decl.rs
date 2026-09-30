/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What a struct or enum declaration declares: a struct one variant of its name, an enum
 * its variants in order, and a struct the labels its fields write.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::adt_enum::form_of;
use super::cx::{Sema, State};
use super::defs::DefId;
use crate::compiler::sema::ty::AdtVariant;
use crate::compiler::syntax::ast::{Item, ItemKind};
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /**
     * Whether `item`, the declaration of `def` named `name` in `module`, is an enum, its
     * variants, and its fields' labels; `None` for a generic one, not checked yet.
     */
    pub(super) fn declared(
        &mut self,
        def: DefId,
        module: DefId,
        item: &'a Item,
        name: &str,
    ) -> Option<(bool, Vec<AdtVariant>, Labels)> {
        match &item.kind {
            ItemKind::Struct(s) if s.generics.is_empty() => {
                self.structs.insert(def, State::Checking);
                let (form, decls) = form_of(&s.fields);
                let (fields, labels) = self.fields(module, decls);
                let name = name.into();
                Some((false, vec![AdtVariant { name, form, fields }], labels))
            }
            ItemKind::Enum(e) if e.generics.is_empty() => {
                self.structs.insert(def, State::Checking);
                Some((true, self.variants(module, &e.variants), Labels::default()))
            }
            _ => None,
        }
    }
}
