/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Applying a glob import, and reporting an import that does not resolve. */

use alloc::string::String;
use alloc::vec::Vec;

use super::PendingImport;
use super::{Binding, BindingKind, DefId, DefKind, Defs, PathError};
use crate::compiler::diag::Diagnostics;

impl<'a> Defs<'a> {
    /** Bind every name `module` shows the glob's module, and say whether any was new. */
    pub(super) fn apply_glob(
        &mut self,
        imp: &PendingImport<'a>,
        module: DefId,
        diags: &mut Diagnostics,
    ) -> bool {
        if self.get(module).map(|d| d.kind) != Some(DefKind::Mod) {
            self.report_import(imp, PathError::NotModule(imp.segs.len()), diags);
            return false;
        }
        let names: Vec<(String, DefId)> = match self.modules.get(&module) {
            Some(m) => m
                .names
                .iter()
                .filter(|(_, b)| {
                    b.kind != BindingKind::Ambiguous && self.visible(b, module, imp.module)
                })
                .map(|(n, b)| (n.clone(), b.def))
                .collect(),
            None => Vec::new(),
        };
        let mut changed = false;
        for (name, def) in names {
            let b = Binding {
                def,
                kind: BindingKind::Glob,
                span: imp.span,
                vis: imp.vis,
            };
            changed |= self.bind(imp.module, &name, b, diags);
        }
        changed
    }
}
