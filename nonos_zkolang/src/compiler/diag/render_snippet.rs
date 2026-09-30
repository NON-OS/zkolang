/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The part of a rendered diagnostic that shows one label: a source line and its underline. */

use alloc::format;
use alloc::string::String;

use super::diagnostic::Label;
use super::render_text::{display_col, expand_tabs};
use crate::compiler::source::SourceFile;

/**
 * Append the line label `l` starts on, at one-based `(line, col)` of `file`, and under it
 * the label's mark and message. `pad` is the gutter: one space per digit of the widest
 * line number.
 */
pub(super) fn push_snippet(
    out: &mut String,
    file: &SourceFile,
    l: &Label,
    at: (usize, usize),
    pad: &str,
) {
    let (line, col) = at;
    let gutter = pad.len();
    let text = file.line_text(line);
    out.push_str(&format!("{:>gutter$} | {}\n", line, expand_tabs(text)));
    let (end_line, end_col) = file.line_col(l.span.hi);
    let start = display_col(text, col);
    let end = if end_line == line {
        display_col(text, end_col).max(start + 1)
    } else {
        display_col(text, text.chars().count() + 1).max(start + 1)
    };
    let mark = if l.primary { '^' } else { '-' };
    let mut under = " ".repeat(start);
    for _ in start..end {
        under.push(mark);
    }
    if !l.message.is_empty() {
        under.push(' ');
        under.push_str(&l.message);
    }
    out.push_str(&format!("{pad} | {under}\n"));
}
