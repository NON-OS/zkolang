/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A module's `impl` blocks: the struct or enum each is for, and its public functions under
 * its own heading.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::header::{header, text};
use super::render::push_doc;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::ast::{ImplDecl, Item, ItemKind, TypeKind, Visibility};

/** The `impl` blocks among `items`. */
pub(super) fn impls(items: &[Item]) -> impl Iterator<Item = (&Item, &ImplDecl)> {
    items.iter().filter_map(|i| match &i.kind {
        ItemKind::Impl(imp) => Some((i, imp)),
        _ => None,
    })
}

/** The name of the struct or enum `imp` is for, if its type is a path. */
pub(super) fn self_name(imp: &ImplDecl) -> Option<&str> {
    match &imp.self_ty.kind {
        TypeKind::Path(p) => Some(p.last_name()),
        _ => None,
    }
}

/** The name of a struct or enum item. */
pub(super) fn item_name(item: &Item) -> Option<&str> {
    match &item.kind {
        ItemKind::Struct(s) => Some(&s.name.name),
        ItemKind::Enum(e) => Some(&e.name.name),
        _ => None,
    }
}

/** The public functions of the `impl` block `imp`, the item `item`, under its heading. */
pub(super) fn impl_block(out: &mut String, map: &SourceMap, (item, imp): (&Item, &ImplDecl)) {
    let public = |i: &&Item| i.vis == Visibility::Public && matches!(i.kind, ItemKind::Fn(_));
    let members: Vec<&Item> = imp.items.iter().filter(public).collect();
    if members.is_empty() {
        return;
    }
    let hi = imp.self_ty.span.hi;
    let before = text(map, item.span, (item.span.lo, imp.self_ty.span.lo));
    let from = before
        .rfind("impl")
        .map_or(String::new(), |i| String::from(&before[i..]));
    let ty = text(map, item.span, (imp.self_ty.span.lo, hi));
    out.push_str(&format!("### `{} {ty}`\n\n", from.trim_end()));
    for m in members {
        if let Some(sig) = header(map, m) {
            out.push_str(&format!("#### `{sig}`\n\n"));
            push_doc(out, m.doc.as_deref());
        }
    }
}
