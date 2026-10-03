/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A label that spans lines, drawn as rustc draws one: its first line with a connector
 * from the margin to where it starts, the lines between with the connector beside them,
 * and its last line with a connector to where it ends, followed by the message. Labels
 * that start within its lines are drawn under their own lines, inside the connector.
 *
 * ```text
 * 2 |       let x = compute(
 *   |  _____________^
 * 3 | |         alpha,
 *   | |         ----- inside
 * 4 | |     );
 *   | |_____^ this call spans lines
 * ```
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::render_gap::push_skipped;
use super::render_gutter::Gutter;
use super::render_line::{push_line, push_marks};
use super::render_multi_end::{connector, push_end};
use super::render_placed::Placed;
use super::render_window::Window;

/** Show the label `p`, which spans lines, and `inner`, the labels that start within it. */
pub(super) fn push_multi(out: &mut String, p: &Placed, inner: &[&Placed], g: &Gutter) {
    let on = |l: usize| -> Vec<&Placed> { inner.iter().filter(|q| q.line == l).copied().collect() };
    let heads = on(p.line);
    let (lo, _) = p.within(p.line);
    let mut subjects: Vec<usize> = heads.iter().map(|q| q.within(p.line).0).collect();
    subjects.push(lo);
    let at = subjects.iter().copied().min().unwrap_or(lo);
    let first = Window::around(p.file.line_text(p.line), at, &subjects);
    g.source(out, p.line, g.plain(), &first.shown);
    push_marks(out, &first, &heads, g, g.plain());
    g.mark(out, " ", &connector(first.col(lo), p.mark(), ""));
    let mut shown = p.line;
    for q in inner {
        if q.line > shown && q.line < p.end_line {
            push_skipped(out, p, shown, q.line, g);
            push_line(out, &on(q.line), g, "| ");
            shown = q.line;
        }
    }
    push_skipped(out, p, shown, p.end_line, g);
    push_end(out, p, &on(p.end_line), g);
}
