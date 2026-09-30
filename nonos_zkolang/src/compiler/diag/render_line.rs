/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Labels that start and end on one line: the line, once, and under it one row per label
 * with its marks and message.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::display::{columns, visible};
use super::render_gutter::Gutter;
use super::render_placed::Placed;
use super::render_window::Window;

/**
 * The columns from the start of one label to the end of another shown with it. With the
 * lead-in before the first, they fit in a window.
 */
const NEAR: usize = 50;

/** Whether `b`, which starts no earlier than `a` on the same line, is shown with it. */
pub(super) fn near(a: &Placed, b: &Placed) -> bool {
    if b.line != a.line || b.is_multiline() {
        return false;
    }
    let ((lo, _), (_, hi)) = (a.within(a.line), b.within(a.line));
    let text = a.file.line_text(a.line);
    hi.saturating_sub(lo) <= 4 * NEAR && text.get(lo..hi).is_some_and(|s| columns(s) <= NEAR)
}

/** Show line `ls[0].line` and a row for each of `ls`, which all lie on it. */
pub(super) fn push_line(out: &mut String, ls: &[&Placed], g: &Gutter) {
    let Some(first) = ls.first() else {
        return;
    };
    let line = first.line;
    let text = first.file.line_text(line);
    let subjects: Vec<usize> = ls.iter().map(|p| p.within(line).0).collect();
    let w = Window::around(text, first.within(line).0, &subjects);
    g.source(out, line, g.plain(), &w.shown);
    for p in ls {
        let (lo, hi) = p.within(line);
        let start = w.col(lo);
        let end = w.col(hi).max(start + 1);
        let mut marks = " ".repeat(start);
        (start..end).for_each(|_| marks.push(p.mark()));
        if !p.label.message.is_empty() {
            marks.push(' ');
            marks.push_str(&visible(&p.label.message));
        }
        g.mark(out, g.plain(), &marks);
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
