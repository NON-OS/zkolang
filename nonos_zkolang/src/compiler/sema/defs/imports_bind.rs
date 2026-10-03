/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One import: what its path names now, and binding it under its name. */

use alloc::vec::Vec;

use super::{Binding, BindingKind, DefId, Defs, PathError, PendingImport};
use crate::compiler::diag::Diagnostics;

impl<'a> Defs<'a> {
    /** What an import's path names now. */
    pub(super) fn resolve_import(&self, imp: &PendingImport<'a>) -> Result<DefId, PathError> {
        let names: Vec<&str> = imp.segs.iter().map(|s| s.name.as_str()).collect();
        self.resolve(imp.module, imp.root, &names)
    }

    /** Bind a named import, `def`, under its alias or its last name. */
    pub(super) fn bind_import(
        &mut self,
        imp: &PendingImport<'a>,
        def: DefId,
        diags: &mut Diagnostics,
    ) {
        match imp.alias.or(imp.segs.last().copied()) {
            Some(name) => {
                let b = Binding {
                    def,
                    kind: BindingKind::Import,
                    span: name.span,
                    vis: imp.vis,
                };
                self.bind(imp.module, &name.name, b, diags);
            }
            None => self.report_import(imp, PathError::BadRoot, diags),
        }
    }
}
