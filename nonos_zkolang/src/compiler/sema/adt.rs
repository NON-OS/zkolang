/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Struct declarations (section 5.2): each lowered once into an entry of the type table,
 * its fields' types and labels with it. A struct that contains itself is reported
 * (E0313), since its values would take no finite number of slots.
 */

use alloc::vec;

use super::cx::{Sema, State};
use super::defs::DefId;
use crate::compiler::sema::ty::{Adt, AdtVariant, Form, TyId, Types};
use crate::compiler::syntax::ast::{Fields, ItemKind};
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /** The type the struct `def` declares, and its fields' labels, each under its index. */
    pub(crate) fn struct_ty(&mut self, def: DefId) -> (TyId, Labels) {
        let none = (Types::ERROR, Labels::default());
        match self.structs.get(&def) {
            Some(State::Done(x)) => return x.clone(),
            Some(State::Checking) => {
                self.contains_itself(def);
                self.structs.insert(def, State::Failed);
                return none;
            }
            Some(State::Failed) => return none,
            _ => {}
        }
        let found = self
            .defs
            .get(def)
            .and_then(|d| Some((d.parent?, d.item?, d.name.clone())));
        let Some((module, item, name)) = found else {
            return none;
        };
        let ItemKind::Struct(s) = &item.kind else {
            return none;
        };
        if !s.generics.is_empty() {
            return none;
        }
        self.structs.insert(def, State::Checking);
        let (form, decls) = match &s.fields {
            Fields::Unit => (Form::Unit, &[][..]),
            Fields::Tuple(f) => (Form::Tuple, &f[..]),
            Fields::Named(f) => (Form::Named, &f[..]),
        };
        let (fields, labels) = self.fields(module, decls);
        if !matches!(self.structs.get(&def), Some(State::Checking)) {
            return none;
        }
        let variants = vec![AdtVariant {
            name: name.clone(),
            form,
            fields,
        }];
        let adt = Adt {
            name,
            def: def.0,
            module: module.0,
            is_enum: false,
            variants,
        };
        let out = (self.types.add_adt(adt), labels);
        self.structs.insert(def, State::Done(out.clone()));
        out
    }
}
