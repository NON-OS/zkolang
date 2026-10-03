/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Binding a name in a module. An item and a named import each claim their name: a second
 * claim is an error (E0201). A glob import yields to them, and two globs that bring one
 * name from different items make it ambiguous.
 */

use alloc::format;
use alloc::string::String;

use super::{Binding, BindingKind, DefId, Defs};
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};

impl<'a> Defs<'a> {
    /** Bind `name` in `module`, and say whether the namespace changed. */
    pub(super) fn bind(
        &mut self,
        module: DefId,
        name: &str,
        b: Binding,
        diags: &mut Diagnostics,
    ) -> bool {
        let Some(m) = self.modules.get_mut(&module) else {
            return false;
        };
        let Some(old) = m.names.get_mut(name) else {
            m.names.insert(String::from(name), b);
            return true;
        };
        let strong = |k: BindingKind| matches!(k, BindingKind::Item | BindingKind::Import);
        match (strong(old.kind), strong(b.kind)) {
            (false, true) => {
                *old = b;
                true
            }
            (true, false) => false,
            (false, false) if old.def == b.def || old.kind == BindingKind::Ambiguous => false,
            (false, false) => {
                old.kind = BindingKind::Ambiguous;
                true
            }
            (true, true) => {
                let d = Diagnostic::error(
                    Code::DUPLICATE_ITEM,
                    format!("the name `{name}` is defined twice in this module"),
                    b.span,
                    "defined again here",
                )
                .with_label(old.span, "first defined here")
                .with_help(
                    "a module has one namespace for its items and imports; rename one of them",
                );
                diags.push(d);
                false
            }
        }
    }
}
