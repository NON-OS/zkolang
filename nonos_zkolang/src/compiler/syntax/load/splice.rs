/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Giving each `mod name;` the items of its file (section 4.2). The children of a module
 * are sought in the directory named after it, whether it is inline, `name.zkl` or
 * `name/mod.zkl`. A module whose file cannot be loaded is reported and left empty.
 */

use alloc::format;
use alloc::vec::Vec;

use super::entry::Loader;
use super::files::join;
use super::load_report::{missing, too_deep};
use crate::compiler::syntax::ast::{Ident, Item, ItemKind, SourceAst};

/** How deep modules may nest, which a directory that contains itself would exceed. */
const MAX_DEPTH: u32 = 64;

impl Loader<'_> {
    /** Load the modules `items` declares, whose files are in `dir`, `depth` modules down. */
    pub(super) fn splice(&mut self, items: &mut [Item], dir: &str, depth: u32) {
        for item in items {
            let ItemKind::Mod(m) = &mut item.kind else {
                continue;
            };
            if m.body.is_none() {
                let ast = if depth < MAX_DEPTH {
                    self.module_file(dir, &m.name)
                } else {
                    self.diags.push(too_deep(&m.name, MAX_DEPTH));
                    None
                };
                let ast = ast.unwrap_or_else(|| SourceAst {
                    file: m.name.span.file,
                    inner_attrs: Vec::new(),
                    inner_doc: None,
                    items: Vec::new(),
                    span: m.name.span,
                });
                m.body = Some(ast.items);
                m.inner_attrs = ast.inner_attrs;
                m.inner_doc = ast.inner_doc;
                m.file = Some(ast.span);
            }
            let child = join(dir, &m.name.name);
            if let Some(body) = &mut m.body {
                self.splice(body, &child, depth + 1);
            }
        }
    }

    /** The file of the module `name` declared in `dir`: one of `name.zkl` and `name/mod.zkl`. */
    fn module_file(&mut self, dir: &str, name: &Ident) -> Option<SourceAst> {
        let flat = join(dir, &format!("{}.zkl", name.name));
        let nested = join(&join(dir, &name.name), "mod.zkl");
        match (self.files.read(&flat), self.files.read(&nested)) {
            (Some(text), None) => Some(self.parse(&flat, text)),
            (None, Some(text)) => Some(self.parse(&nested, text)),
            (both, _) => {
                self.diags
                    .push(missing(name, (&flat, &nested), both.is_some()));
                None
            }
        }
    }
}
