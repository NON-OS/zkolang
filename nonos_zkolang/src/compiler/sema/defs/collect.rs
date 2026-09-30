/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Collecting a program's items into the table and their names into their modules. */

use alloc::string::String;
use alloc::vec::Vec;

use super::PendingImport;
use super::{Def, DefId, DefKind, Defs, Module};
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
        Defs::collect_with(ast, None, false, diags)
    }

    /**
     * As `collect`, with the items marked `#[cfg(test)]` too when `testing`, and the crate
     * `std` from `std` beside it.
     */
    pub fn collect_with(
        ast: &'a SourceAst,
        std: Option<&'a SourceAst>,
        testing: bool,
        diags: &mut Diagnostics,
    ) -> (Defs<'a>, Vec<PendingImport<'a>>) {
        let mut defs = Defs {
            testing,
            ..Defs::default()
        };
        let mut imports = Vec::new();
        defs.collect_crate("crate", ast, &mut imports, diags);
        if let Some(std) = std {
            defs.std = Some(defs.collect_crate("std", std, &mut imports, diags));
        }
        (defs, imports)
    }

    /** Collect the crate `ast` as a root named `name`, returning the root. */
    fn collect_crate(
        &mut self,
        name: &str,
        ast: &'a SourceAst,
        imports: &mut Vec<PendingImport<'a>>,
        diags: &mut Diagnostics,
    ) -> DefId {
        let root = self.push(Def {
            kind: DefKind::Mod,
            name: String::from(name),
            span: ast.span,
            parent: None,
            vis: Visibility::Public,
            item: None,
        });
        self.modules.insert(root, Module::default());
        self.collect_items(root, &ast.items, imports, diags);
        root
    }
}
