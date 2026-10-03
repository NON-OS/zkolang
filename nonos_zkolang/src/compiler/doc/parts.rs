/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What a reference lists under an item: a struct's public named fields, which a tuple
 * struct's signature shows already, and an enum's variants, each with its doc.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::header::text;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::ast::{Fields, Item, ItemKind, Visibility};

/** The fields or variants of `item`, as a list. */
pub(super) fn parts(out: &mut String, map: &SourceMap, item: &Item) {
    let mut list = Vec::new();
    match &item.kind {
        ItemKind::Struct(s) => {
            if let Fields::Named(fs) = &s.fields {
                for f in fs.iter().filter(|f| f.vis == Visibility::Public) {
                    let lo = f.name.as_ref().map_or(f.ty.span.lo, |n| n.span.lo);
                    let t = text(map, f.ty.span, (lo, f.ty.span.hi));
                    list.push((format!("pub {t}"), f.doc.as_deref()));
                }
            }
        }
        ItemKind::Enum(e) => {
            for v in &e.variants {
                list.push((
                    text(map, v.span, (v.name.span.lo, v.span.hi)),
                    v.doc.as_deref(),
                ));
            }
        }
        _ => {}
    }
    for (t, doc) in &list {
        let line = match doc {
            Some(d) => format!("- `{t}`: {}\n", d.trim()),
            None => format!("- `{t}`\n"),
        };
        out.push_str(&line);
    }
    if !list.is_empty() {
        out.push('\n');
    }
}
