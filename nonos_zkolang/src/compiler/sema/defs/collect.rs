/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Collecting a program's items into the table and their names into their modules. */

use alloc::string::String;
use alloc::vec::Vec;

use super::PendingImport;
use super::{Def, DefKind, Defs, Module};
use crate::compiler::diag::Diagnostics;
use crate::compiler::syntax::ast::{SourceAst, Visibility};

impl<'a> Defs<'a> {
    /**
     * The items of `ast`, each module's declared names, and the imports left to resolve.
     * Two items of one name in a module are reported (E0201).
     */
    pub fn collect(
        ast: &'a SourceAst,
        diags: &mut Diagnostics,
    ) -> (Defs<'a>, Vec<PendingImport<'a>>) {
        Defs::collect_with(ast, false, diags)
    }

    /** As `collect`, with the items marked `#[cfg(test)]` too when `testing`. */
    pub fn collect_with(
        ast: &'a SourceAst,
        testing: bool,
        diags: &mut Diagnostics,
    ) -> (Defs<'a>, Vec<PendingImport<'a>>) {
        let mut defs = Defs {
            testing,
            ..Defs::default()
        };
        let root = defs.push(Def {
            kind: DefKind::Mod,
            name: String::from("crate"),
            span: ast.span,
            parent: None,
            vis: Visibility::Public,
            item: None,
        });
        defs.modules.insert(root, Module::default());
        let mut imports = Vec::new();
        defs.collect_items(root, &ast.items, &mut imports, diags);
        (defs, imports)
    }
}
