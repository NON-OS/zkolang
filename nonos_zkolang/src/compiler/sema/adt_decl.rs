/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What a struct or enum declaration declares for one list of generic arguments: a struct
 * one variant of its name, an enum its variants in order, and a struct the labels its
 * fields write. Each generic parameter stands for its argument while the fields lower.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::adt_enum::form_of;
use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::sema::ty::{AdtVariant, GenArg};
use crate::compiler::syntax::ast::{Item, ItemKind};
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /**
     * Whether `item`, named `name` in `module`, is an enum, its variants and its fields'
     * labels, for `args`; `None` if `item` is no struct or enum or takes other arguments.
     */
    pub(super) fn declared(
        &mut self,
        module: DefId,
        item: &'a Item,
        name: &str,
        args: &[GenArg],
    ) -> Option<(bool, Vec<AdtVariant>, Labels)> {
        let params = match &item.kind {
            ItemKind::Struct(s) => &s.generics,
            ItemKind::Enum(e) => &e.generics,
            _ => return None,
        };
        let outer = self.bind_generics(params, args)?;
        let out = match &item.kind {
            ItemKind::Struct(s) => {
                let (form, decls) = form_of(&s.fields);
                let (fields, labels) = self.fields(module, decls);
                let name = name.into();
                (false, vec![AdtVariant { name, form, fields }], labels)
            }
            ItemKind::Enum(e) => (true, self.variants(module, &e.variants), Labels::default()),
            _ => (false, Vec::new(), Labels::default()),
        };
        self.generics = outer;
        Some(out)
    }
}
