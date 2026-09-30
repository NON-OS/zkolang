/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A crate's reference, from its doc comments (section 17.2): each module that is public,
 * with its own doc, then each public item's signature and doc in the order written, the
 * fields and variants of structs and enums, and the public functions of `impl` blocks.
 */

use alloc::format;
use alloc::string::String;

use super::header::header;
use crate::compiler::source::SourceMap;
use alloc::vec::Vec;

use super::items::{impls, item_name, self_name};
use crate::compiler::syntax::ast::{Item, ItemKind, SourceAst, Visibility};

/** The reference of the crate `ast`, named `name`, as Markdown. */
pub fn render_doc(map: &SourceMap, name: &str, ast: &SourceAst) -> String {
    let mut out =
        format!("<!-- The reference of `{name}`, made by `zkolang doc`. -->\n\n# `{name}`\n\n");
    push_doc(&mut out, ast.inner_doc.as_deref());
    module(&mut out, map, name, &ast.items);
    out
}

/** The items of the module at `path`, then its public modules. */
fn module(out: &mut String, map: &SourceMap, path: &str, items: &[Item]) {
    for item in items.iter().filter(|i| i.vis == Visibility::Public) {
        let Some(sig) = header(map, item) else {
            continue;
        };
        out.push_str(&format!("### `{sig}`\n\n"));
        push_doc(out, item.doc.as_deref());
        super::parts::parts(out, map, item);
        for (i, imp) in impls(items).filter(|(_, imp)| self_name(imp) == item_name(item)) {
            super::items::impl_block(out, map, (i, imp));
        }
    }
    let listed: Vec<&str> = items
        .iter()
        .filter(|i| i.vis == Visibility::Public)
        .filter_map(item_name)
        .collect();
    for (i, imp) in
        impls(items).filter(|(_, imp)| self_name(imp).is_none_or(|n| !listed.contains(&n)))
    {
        super::items::impl_block(out, map, (i, imp));
    }
    for item in items.iter().filter(|i| i.vis == Visibility::Public) {
        if let ItemKind::Mod(m) = &item.kind {
            let inner = format!("{path}::{}", m.name.name);
            out.push_str(&format!("## `{inner}`\n\n"));
            push_doc(out, m.inner_doc.as_deref().or(item.doc.as_deref()));
            module(out, map, &inner, m.body.as_deref().unwrap_or(&[]));
        }
    }
}

/** A doc comment's text, if there is one, as a paragraph. */
pub(super) fn push_doc(out: &mut String, doc: Option<&str>) {
    if let Some(d) = doc.map(str::trim).filter(|d| !d.is_empty()) {
        out.push_str(d);
        out.push_str("\n\n");
    }
}
