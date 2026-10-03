/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The parts of a label over lines after its first line: the lines between, and its end. */

use alloc::string::String;
use alloc::vec::Vec;

use super::display::visible;
use super::render_gutter::Gutter;
use super::render_line::push_marks;
use super::render_placed::Placed;
use super::render_window::Window;

/** The last line of the label `p`, the labels `tails` that start on it, and its end. */
pub(super) fn push_end(out: &mut String, p: &Placed, tails: &[&Placed], g: &Gutter) {
    let (_, hi) = p.within(p.end_line);
    let last_char = hi.saturating_sub(1);
    let subjects: Vec<usize> = tails.iter().map(|q| q.within(p.end_line).0).collect();
    let at = subjects
        .iter()
        .copied()
        .min()
        .unwrap_or(last_char)
        .min(last_char);
    let last = Window::around(p.file.line_text(p.end_line), at, &subjects);
    g.source(out, p.end_line, "| ", &last.shown);
    push_marks(out, &last, tails, g, "| ");
    let end = last.col(last_char);
    let message = visible(&p.label.message);
    g.mark(out, "|", &connector(end, p.mark(), &message));
}

/** A run of `_` from the connector column to display column `col`, the mark, a message. */
pub(super) fn connector(col: usize, mark: char, message: &str) -> String {
    let mut s = "_".repeat(col + 1);
    s.push(mark);
    if !message.is_empty() {
        s.push(' ');
        s.push_str(message);
    }
    s
}
