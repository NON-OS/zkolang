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
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::diagnostic::{Diagnostic, Label, Severity};
use super::render_snippet::push_snippet;
use super::render_text::digits;
use crate::compiler::source::SourceMap;

/** Render one diagnostic over the source map. */
pub fn render(map: &SourceMap, d: &Diagnostic) -> String {
    let kind = match d.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    };
    let mut out = format!("{kind}[{}]: {}\n", d.code.0, d.message);
    let mut labels: Vec<&Label> = d.labels.iter().collect();
    labels.sort_by_key(|l| (!l.primary, l.span.file, l.span.lo));
    let gutter = labels
        .iter()
        .filter_map(|l| map.locate(l.span).map(|(_, line, _)| digits(line)))
        .max()
        .unwrap_or(1);
    let pad = " ".repeat(gutter);
    let mut current_file = None;
    for l in &labels {
        let Some(file) = map.file(l.span.file) else {
            continue;
        };
        let (line, col) = file.line_col(l.span.lo);
        if current_file != Some(l.span.file) {
            out.push_str(&format!("{pad}--> {}:{line}:{col}\n", file.name));
            out.push_str(&format!("{pad} |\n"));
            current_file = Some(l.span.file);
        }
        push_snippet(&mut out, file, l, (line, col), &pad);
    }
    if !d.notes.is_empty() || d.help.is_some() {
        out.push_str(&format!("{pad} |\n"));
    }
    for n in &d.notes {
        out.push_str(&format!("{pad} = note: {n}\n"));
    }
    if let Some(h) = &d.help {
        out.push_str(&format!("{pad} = help: {h}\n"));
    }
    out
}
