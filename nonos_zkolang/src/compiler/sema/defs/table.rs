/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The table of a program's items and the namespaces of its modules. */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::{Def, DefId, Module};

/** Every named item of a program, the root module first, and each module's names. */
#[derive(Clone, Debug, Default)]
pub struct Defs<'a> {
    pub defs: Vec<Def<'a>>,
    pub modules: BTreeMap<DefId, Module>,
    /** Whether items marked `#[cfg(test)]` are compiled (section 16). */
    pub testing: bool,
}

impl<'a> Defs<'a> {
    /** The root module, the crate. */
    pub const ROOT: DefId = DefId(0);

    /** The item `id` names. */
    pub fn get(&self, id: DefId) -> Option<&Def<'a>> {
        self.defs.get(id.0 as usize)
    }

    /** Add an item and return its id. */
    pub(crate) fn push(&mut self, def: Def<'a>) -> DefId {
        let id = DefId(u32::try_from(self.defs.len()).unwrap_or(u32::MAX));
        self.defs.push(def);
        id
    }

    /** The module that declares `id`, or `None` for the root. */
    pub fn parent(&self, id: DefId) -> Option<DefId> {
        self.get(id).and_then(|d| d.parent)
    }
}
