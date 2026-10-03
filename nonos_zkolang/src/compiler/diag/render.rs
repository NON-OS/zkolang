/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Render a diagnostic as text in the style of rustc:
 *
 * ```text
 * error[E0300]: mismatched types
 *   --> src/main.zkl:3:17
 *    |
 *  3 |     let x: u8 = true;
 *    |                 ^^^^ expected `u8`, found `bool`
 *    |
 *    = help: ...
 * ```
 *
 * The primary label's file comes first, introduced by `-->`; any other file a label
 * points into follows, introduced by `:::`.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::diagnostic::{Diagnostic, Severity};
use super::display::visible;
use super::render_files::push_files;
use super::render_gutter::Gutter;
use super::render_placed::Placed;
use crate::compiler::source::SourceMap;

/** Render one diagnostic over the source map. */
pub fn render(map: &SourceMap, d: &Diagnostic) -> String {
    let kind = match d.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    };
    let mut out = format!("{kind}[{}]: {}\n", d.code.0, visible(&d.message));
    let mut placed: Vec<Placed> = d
        .labels
        .iter()
        .filter_map(|l| Placed::new(map, l))
        .collect();
    placed.sort_by_key(|p| !p.label.primary);
    let max_line = placed.iter().map(|p| p.end_line).max().unwrap_or(1);
    let g = Gutter::new(max_line, placed.iter().any(Placed::is_multiline));
    push_files(&mut out, &placed, &g);
    if !d.notes.is_empty() || d.help.is_some() {
        g.rule(&mut out);
    }
    for n in &d.notes {
        out.push_str(&format!("{} = note: {}\n", g.pad, visible(n)));
    }
    if let Some(h) = &d.help {
        out.push_str(&format!("{} = help: {}\n", g.pad, visible(h)));
    }
    out
}
