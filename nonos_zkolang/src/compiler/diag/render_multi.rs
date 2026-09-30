/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A label that spans lines, drawn as rustc draws one: its first line with a connector
 * from the margin to where it starts, and its last line with a connector from the margin
 * to where it ends, followed by the message.
 *
 * ```text
 * 2 |       let x = 1 + (return
 *   |  __________________^
 * 3 | |         5);
 *   | |__________^ inside a larger expression
 * ```
 */

use alloc::string::String;

use super::display::visible;
use super::render_gutter::Gutter;
use super::render_placed::Placed;
use super::render_window::Window;

/** Show the label `p`, which spans lines. */
pub(super) fn push_multi(out: &mut String, p: &Placed, g: &Gutter) {
    let (lo, _) = p.within(p.line);
    let first = Window::around(p.file.line_text(p.line), lo, &[lo]);
    g.source(out, p.line, g.plain(), &first.shown);
    let start = first.col(lo);
    g.mark(out, " ", &connector(start, p.mark(), ""));
    if p.end_line == p.line + 2 {
        let text = p.file.line_text(p.line + 1);
        g.source(out, p.line + 1, "| ", &Window::around(text, 0, &[]).shown);
    } else if p.end_line > p.line + 2 {
        g.elision(out);
    }
    let (_, hi) = p.within(p.end_line);
    let last_char = hi.saturating_sub(1);
    let last = Window::around(p.file.line_text(p.end_line), last_char, &[]);
    g.source(out, p.end_line, "| ", &last.shown);
    let end = last.col(last_char);
    g.mark(
        out,
        "|",
        &connector(end, p.mark(), &visible(&p.label.message)),
    );
}

/** A run of `_` from the connector column to display column `col`, the mark, a message. */
fn connector(col: usize, mark: char, message: &str) -> String {
    let mut s = "_".repeat(col + 1);
    s.push(mark);
    if !message.is_empty() {
        s.push(' ');
        s.push_str(message);
    }
    s
}
