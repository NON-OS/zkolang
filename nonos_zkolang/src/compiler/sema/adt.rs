/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Struct and enum declarations (section 5.2): each instance, for one list of generic
 * arguments, lowered once into an entry of the type table, its fields' types and labels
 * with it. One that contains itself is reported (E0313), since its values would take no
 * finite number of slots; so is one whose instances contain ever larger instances.
 */

use alloc::vec::Vec;

use super::cx::{Sema, State};
use super::defs::DefId;
use crate::compiler::sema::ty::{Adt, GenArg, TyId, Types};
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /** The type the struct or enum `def`, which takes no generic arguments, declares. */
    pub(crate) fn struct_ty(&mut self, def: DefId) -> (TyId, Labels) {
        self.adt_ty(def, &[])
    }

    /**
     * The instance of the struct or enum `def` for `args`, and the labels a struct's
     * fields write, each under its index. The caller has checked the arguments' number.
     */
    pub(crate) fn adt_ty(&mut self, def: DefId, args: &[GenArg]) -> (TyId, Labels) {
        let none = (Types::ERROR, Labels::default());
        let key = (def, args.to_vec());
        if let Some(known) = self.adt_known(&key) {
            return known;
        }
        let found = self
            .defs
            .get(def)
            .and_then(|d| Some((d.parent?, d.item?, d.name.clone())));
        let Some((module, item, name)) = found else {
            return none;
        };
        self.structs.insert(key.clone(), State::Checking);
        let Some((is_enum, variants, labels)) = self.declared(module, item, &name, args) else {
            self.structs.insert(key, State::Failed);
            return none;
        };
        if !matches!(self.structs.get(&key), Some(State::Checking)) {
            return none;
        }
        let args: Vec<GenArg> = args.to_vec();
        let (def, module) = (def.0, module.0);
        let adt = Adt {
            name,
            def,
            module,
            args,
            is_enum,
            variants,
        };
        let out = (self.types.add_adt(adt), labels);
        self.structs.insert(key, State::Done(out.clone()));
        out
    }
}
