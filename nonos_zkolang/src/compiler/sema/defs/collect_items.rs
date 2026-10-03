/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Collecting the items of one module, and of the modules inline in it. An item marked
 * `#[cfg(test)]` is left out unless testing, and always in a dependency.
 */

use alloc::vec::Vec;

use super::use_flatten::{flatten, Prefix};
use super::{Binding, BindingKind, Def, DefId, DefKind, Defs, Module, PendingImport};
use crate::compiler::diag::Diagnostics;
use crate::compiler::sema::attr_query::cfg_test;
use crate::compiler::syntax::ast::{Item, ItemKind, PathRoot};

impl<'a> Defs<'a> {
    pub(super) fn collect_items(
        &mut self,
        module: DefId,
        items: &'a [Item],
        imports: &mut Vec<PendingImport<'a>>,
        diags: &mut Diagnostics,
    ) {
        for item in items {
            if !self.testing_in(module) && cfg_test(&item.attrs) {
                continue;
            }
            let kind = match &item.kind {
                ItemKind::Use(tree) => {
                    let prefix = Prefix {
                        root: PathRoot::Plain,
                        segs: Vec::new(),
                    };
                    flatten(tree, &prefix, module, item.vis, imports, diags);
                    continue;
                }
                ItemKind::Impl(_) => continue,
                ItemKind::Fn(_) => DefKind::Fn,
                ItemKind::Const(_) => DefKind::Const,
                ItemKind::TypeAlias(_) => DefKind::Alias,
                ItemKind::Struct(_) => DefKind::Struct,
                ItemKind::Enum(_) => DefKind::Enum,
                ItemKind::Mod(_) => DefKind::Mod,
            };
            let Some(name) = item.name() else {
                continue;
            };
            let def = Def {
                kind,
                name: name.name.clone(),
                span: name.span,
                parent: Some(module),
                vis: item.vis,
                item: Some(item),
            };
            let id = self.push(def);
            let binding = Binding {
                def: id,
                kind: BindingKind::Item,
                span: name.span,
                vis: item.vis,
            };
            self.bind(module, &name.name, binding, diags);
            if let ItemKind::Mod(m) = &item.kind {
                self.modules.insert(id, Module::default());
                if let Some(body) = &m.body {
                    self.collect_items(id, body, imports, diags);
                }
            }
        }
    }
}
