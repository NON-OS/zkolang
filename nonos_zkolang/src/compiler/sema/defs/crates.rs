/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The crates of a program (section 4.4): the one being compiled, whose root is
 * `Defs::ROOT`, its dependencies, and `std`, each a root of its own. `crate::` names the
 * root of the crate a path is written in; a first name no module scope binds may name a
 * dependency of that crate, `std`, or a name of the prelude (section 18.1).
 */

use super::{DefId, Defs, Module};
use crate::compiler::syntax::ast::ItemKind;

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

    /**
     * What the first name of a path written in `from` names when no module scope binds
     * it: a dependency of `from`'s crate, the crate `std`, else a name the prelude exports.
     */
    pub(super) fn outside(&self, from: DefId, name: &str) -> Option<DefId> {
        let dep = self
            .deps
            .get(&self.crate_of(from))
            .and_then(|d| d.get(name));
        if let Some(&d) = dep {
            return Some(d);
        }
        if name == "std" {
            return self.std;
        }
        self.prelude()?.names.get(name).map(|b| b.def)
    }

    /** Whether items marked `#[cfg(test)]` are compiled in `module`: in the root crate, when testing. */
    pub fn testing_in(&self, module: DefId) -> bool {
        self.testing && self.crate_of(module) == Defs::ROOT
    }

    /** The prelude, the module `std::prelude`, when the standard library is loaded. */
    pub(super) fn prelude(&self) -> Option<&Module> {
        let std = self.modules.get(&self.std?)?;
        self.modules.get(&std.names.get("prelude")?.def)
    }

    /** The enum of the prelude that has a variant named `name`, if one has. */
    pub fn prelude_variant(&self, name: &str) -> Option<DefId> {
        self.prelude()?.names.values().map(|b| b.def).find(|&d| {
            let item = self.get(d).and_then(|x| x.item).map(|i| &i.kind);
            matches!(item, Some(ItemKind::Enum(e)) if e.variants.iter().any(|v| v.name.name == name))
        })
    }
}
