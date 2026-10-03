/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Source lines between shown lines. A single line is shown; more are left out, and a row
 * of `...` stands for them, with the connector beside it inside a label over lines.
 */

use alloc::format;
use alloc::string::String;

use super::render_gutter::Gutter;
use super::render_placed::Placed;
use super::render_window::Window;

impl Gutter {
    /** The row that stands for source lines left out. */
    pub(super) fn elision(&self, out: &mut String) {
        out.push_str("...\n");
    }

    /** The row that stands for source lines left out inside a label over lines. */
    pub(super) fn elision_in(&self, out: &mut String) {
        let w = self.pad.len() + 3;
        out.push_str(&format!("{:<w$}|\n", "..."));
    }
}

/** Show the one line of `file` between two shown lines, or mark the lines left out. */
pub(super) fn push_gap(out: &mut String, p: &Placed, last: Option<usize>, g: &Gutter) {
    let Some(last) = last else {
        return;
    };
    if p.line == last + 2 {
        let text = p.file.line_text(last + 1);
        g.source(
            out,
            last + 1,
            g.plain(),
            &Window::around(text, 0, &[]).shown,
        );
    } else if p.line > last + 2 {
        g.elision(out);
    }
}

/**
 * Between shown lines `from` and `to` of the label `p`: the one line between, or the row
 * that marks lines left out, with the connector beside it.
 */
pub(super) fn push_skipped(out: &mut String, p: &Placed, from: usize, to: usize, g: &Gutter) {
    if to == from + 2 {
        let text = p.file.line_text(from + 1);
        g.source(out, from + 1, "| ", &Window::around(text, 0, &[]).shown);
    } else if to > from + 2 {
        g.elision_in(out);
    }
}
