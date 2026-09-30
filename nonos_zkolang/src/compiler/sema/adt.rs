/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Struct and enum declarations (section 5.2): each lowered once into an entry of the type
 * table, its fields' types and labels with it. One that contains itself is reported
 * (E0313), since its values would take no finite number of slots.
 */

use super::cx::{Sema, State};
use super::defs::DefId;
use crate::compiler::sema::ty::{Adt, TyId, Types};
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /**
     * The type the struct or enum `def` declares, and the labels a struct's fields write,
     * each under its index.
     */
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
        let Some((is_enum, variants, labels)) = self.declared(def, module, item, &name) else {
            return none;
        };
        if !matches!(self.structs.get(&def), Some(State::Checking)) {
            return none;
        }
        let adt = Adt {
            name,
            def: def.0,
            module: module.0,
            is_enum,
            variants,
        };
        let out = (self.types.add_adt(adt), labels);
        self.structs.insert(def, State::Done(out.clone()));
        out
    }
}
