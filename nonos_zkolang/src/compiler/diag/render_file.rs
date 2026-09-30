/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The labels of one diagnostic that fall in one file, shown in line order. Lines between
 * two shown lines are left out and marked, except a single line, which is shown.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::display::visible;
use super::render_gutter::Gutter;
use super::render_line::{near, push_gap, push_line};
use super::render_multi::push_multi;
use super::render_placed::Placed;

/**
 * Show every file `placed` points into, the first label's file first. Each is introduced
 * by its name and the line and column of its first label.
 */
pub(super) fn push_files(out: &mut String, placed: &[Placed], g: &Gutter) {
    let mut files = Vec::new();
    for p in placed {
        if !files.contains(&p.label.span.file) {
            files.push(p.label.span.file);
        }
    }
    for (i, &f) in files.iter().enumerate() {
        let here: Vec<&Placed> = placed.iter().filter(|p| p.label.span.file == f).collect();
        let Some(head) = here.first() else {
            continue;
        };
        let arrow = if i == 0 { "-->" } else { ":::" };
        let name = visible(&head.file.name);
        let at = format!("{}{arrow} {name}:{}:{}\n", g.pad, head.line, head.col);
        out.push_str(&at);
        g.rule(out);
        push_file(out, here, g);
    }
}

/** Show `ls`, the labels in one file, under the margin `g`. */
fn push_file(out: &mut String, mut ls: Vec<&Placed>, g: &Gutter) {
    ls.sort_by_key(|p| (p.line, p.label.span.lo));
    let mut last: Option<usize> = None;
    let mut i = 0;
    while let Some(p) = ls.get(i) {
        push_gap(out, p, last, g);
        if p.is_multiline() {
            push_multi(out, p, g);
            last = Some(p.end_line);
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while ls.get(j).is_some_and(|q| near(p, q)) {
            j += 1;
        }
        push_line(out, ls.get(i..j).unwrap_or(&[]), g);
        last = Some(p.line);
        i = j;
    }
}
