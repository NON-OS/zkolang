/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Resolving an item path (section 4.4): its root picks the module to start at, each
 * segment but the last names a module, and every name must be visible from where the
 * path is written. A first name the module does not bind may name a crate.
 */

use super::{BindingKind, DefId, DefKind, Defs};
use crate::compiler::syntax::ast::PathRoot;

/** Why a path did not resolve, with the index of the segment at fault. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PathError {
    /** No item of that name where the path looks. */
    Unresolved(usize),
    /** The segment before this one names an item that is not a module. */
    NotModule(usize),
    /** The item is not visible from where the path is written. */
    Private(usize),
    /** Two globs bring the name from different items. */
    Ambiguous(usize),
    /** `super` in the root module, or `Self` outside an impl block. */
    BadRoot,
}

impl<'a> Defs<'a> {
    /** The item the path `segs` after `root` names, from module `from`. */
    pub fn resolve(&self, from: DefId, root: PathRoot, segs: &[&str]) -> Result<DefId, PathError> {
        let mut module = match root {
            PathRoot::Plain | PathRoot::SelfModule => from,
            PathRoot::Crate => self.crate_of(from),
            PathRoot::Super => self.parent(from).ok_or(PathError::BadRoot)?,
            PathRoot::SelfType => return Err(PathError::BadRoot),
        };
        if segs.is_empty() {
            return if root == PathRoot::Plain {
                Err(PathError::Unresolved(0))
            } else {
                Ok(module)
            };
        }
        let mut def = module;
        for (i, name) in segs.iter().enumerate() {
            if i > 0 {
                if self.get(def).map(|d| d.kind) != Some(DefKind::Mod) {
                    return Err(PathError::NotModule(i));
                }
                module = def;
            }
            let found = self.modules.get(&module).and_then(|m| m.names.get(*name));
            let Some(b) = found else {
                let crate_root = (i == 0 && root == PathRoot::Plain).then(|| self.outside(name));
                def = crate_root.flatten().ok_or(PathError::Unresolved(i))?;
                continue;
            };
            if b.kind == BindingKind::Ambiguous {
                return Err(PathError::Ambiguous(i));
            }
            let own_scope = i == 0 && root == PathRoot::Plain;
            if !own_scope && !self.visible(b, module, from) {
                return Err(PathError::Private(i));
            }
            def = b.def;
        }
        Ok(def)
    }
}
