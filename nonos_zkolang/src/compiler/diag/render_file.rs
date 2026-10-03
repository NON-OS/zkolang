/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The labels of one diagnostic that fall in one file, shown in line order. Lines between
 * two shown lines are left out and marked, except a single line, which is shown.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::render_gap::push_gap;
use super::render_gutter::Gutter;
use super::render_line::{near, push_line};
use super::render_multi::push_multi;
use super::render_placed::Placed;

/** Show `ls`, the labels in one file, under the margin `g`. */
pub(super) fn push_file(out: &mut String, mut ls: Vec<&Placed>, g: &Gutter) {
    /* A label over lines comes first on its line, so the labels beside it are drawn in it. */
    ls.sort_by_key(|p| (p.line, !p.is_multiline(), p.label.span.lo));
    let mut last: Option<usize> = None;
    let mut i = 0;
    while let Some(p) = ls.get(i) {
        push_gap(out, p, last, g);
        if p.is_multiline() {
            let mut j = i + 1;
            while ls.get(j).is_some_and(|q| q.line <= p.end_line) {
                j += 1;
            }
            push_multi(out, p, ls.get(i + 1..j).unwrap_or(&[]), g);
            last = Some(p.end_line);
            i = j;
            continue;
        }
        let mut j = i + 1;
        while ls.get(j).is_some_and(|q| near(p, q)) {
            j += 1;
        }
        push_line(out, ls.get(i..j).unwrap_or(&[]), g, g.plain());
        last = Some(p.line);
        i = j;
    }
}
