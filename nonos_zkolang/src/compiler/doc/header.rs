/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The signature of an item as its source writes it, from its name to its body, with its
 * whitespace made single spaces, so what a reference shows is what the code says.
 */

use alloc::format;
use alloc::string::String;

use crate::compiler::source::{SourceMap, Span};
use crate::compiler::syntax::ast::{Item, ItemKind, UseTree, Visibility};

/** The source text from `lo` to `hi` of the file of `at`, its whitespace made single spaces. */
pub(super) fn text(map: &SourceMap, at: Span, (lo, hi): (u32, u32)) -> String {
    let raw = map.snippet(Span::new(at.file, lo, hi));
    let mut out = String::with_capacity(raw.len());
    for word in raw.split_whitespace() {
        if word.starts_with([')', ']', '>']) && out.ends_with(',') {
            out.pop();
        }
        let tight = out.ends_with(['(', '[', '<']) || word.starts_with([')', ']', ',', '>', ';']);
        if !out.is_empty() && !tight {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}

/** The signature of `item`, or `None` for an item a reference does not list on its own. */
pub(super) fn header(map: &SourceMap, item: &Item) -> Option<String> {
    let pub_ = if item.vis == Visibility::Public {
        "pub "
    } else {
        ""
    };
    let (kw, name, end) = match &item.kind {
        ItemKind::Fn(f) => {
            let kw = if f.is_const { "const fn" } else { "fn" };
            (kw, f.name.span, f.body.span.lo)
        }
        ItemKind::Const(c) => ("const", c.name.span, c.value.span.lo),
        ItemKind::Struct(s) => ("struct", s.name.span, item.span.hi),
        ItemKind::Enum(e) => ("enum", e.name.span, item.span.hi),
        ItemKind::TypeAlias(t) => ("type", t.name.span, item.span.hi),
        ItemKind::Use(tree) => {
            let at = match tree {
                UseTree::Single { span, .. }
                | UseTree::Glob { span, .. }
                | UseTree::Nested { span, .. } => *span,
            };
            return Some(format!("{pub_}use {}", text(map, at, (at.lo, at.hi))));
        }
        ItemKind::Mod(_) | ItemKind::Impl(_) => return None,
    };
    let mut sig = text(map, name, (name.lo, end));
    let keep = matches!(item.kind, ItemKind::Fn(_) | ItemKind::TypeAlias(_));
    if let Some(cut) = sig.find(['{', '=']).filter(|_| !keep) {
        sig.truncate(cut);
    }
    let sig = sig.trim_end_matches([' ', ';', '{']).trim_end();
    Some(format!("{pub_}{kw} {sig}"))
}
