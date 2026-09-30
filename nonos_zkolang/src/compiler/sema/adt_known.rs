/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * An instance of a struct or enum already met: lowered, failed, or being lowered, which
 * is a cycle (E0313). So is an instance opened inside more instances of its item than
 * `NESTED`, which only ever larger instances do.
 */

use super::cx::{Instance, Sema, State};
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::tir::Labels;

/** How many instances of one struct or enum may be open, one inside another. */
const NESTED: usize = 64;

impl<'a> Sema<'a> {
    /** The instance `key`, if it is met already; `None` if it is to be lowered now. */
    pub(super) fn adt_known(&mut self, key: &Instance) -> Option<(TyId, Labels)> {
        let none = (Types::ERROR, Labels::default());
        let def = key.0;
        let open = self
            .structs
            .iter()
            .filter(|(k, v)| k.0 == def && matches!(v, State::Checking))
            .count();
        match self.structs.get(key) {
            Some(State::Done(x)) => Some(x.clone()),
            Some(State::Failed) => Some(none),
            Some(State::Checking) => {
                self.contains_itself(def);
                self.structs.insert(key.clone(), State::Failed);
                Some(none)
            }
            _ if open >= NESTED => {
                self.contains_itself(def);
                self.structs.insert(key.clone(), State::Failed);
                Some(none)
            }
            _ => None,
        }
    }
}
