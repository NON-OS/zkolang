/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The crates of a program (section 4.4): the one being compiled, whose root is
 * `Defs::ROOT`, and `std`, a root of its own. `crate::` names the root of the crate a
 * path is written in; a first name no module scope binds may name `std`.
 */

use super::{DefId, Defs};

impl<'a> Defs<'a> {
    /** The root of the crate that holds `id`. */
    pub fn crate_of(&self, id: DefId) -> DefId {
        let mut at = id;
        /* Parents form a tree, so the walk ends; the bound only guards a malformed table. */
        for _ in 0..=self.defs.len() {
            match self.parent(at) {
                Some(p) => at = p,
                None => break,
            }
        }
        at
    }

    /** What the first name of a path names when no module scope binds it: a crate. */
    pub(super) fn outside(&self, name: &str) -> Option<DefId> {
        match name {
            "std" => self.std,
            _ => None,
        }
    }
}
