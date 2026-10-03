/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Visibility (section 4.3): a name is usable in its module and the modules inside it, and a
 * `pub` one wherever its module is.
 */

use super::{Binding, DefId, Defs};
use crate::compiler::syntax::ast::Visibility;

impl<'a> Defs<'a> {
    /** Whether `module` is `ancestor` or lies inside it. */
    pub fn within(&self, module: DefId, ancestor: DefId) -> bool {
        let mut at = Some(module);
        /* Parents form a tree, so the walk ends; the bound only guards a malformed table. */
        for _ in 0..=self.defs.len() {
            match at {
                Some(m) if m == ancestor => return true,
                Some(m) => at = self.parent(m),
                None => return false,
            }
        }
        false
    }

    /** Whether the binding `b` of `module` may be used from module `from`. */
    pub fn visible(&self, b: &Binding, module: DefId, from: DefId) -> bool {
        b.vis == Visibility::Public || self.within(from, module)
    }
}
