/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The files a diagnostic's labels fall in, each introduced by its name and a position. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::display::visible;
use super::render_file::push_file;
use super::render_gutter::Gutter;
use super::render_placed::Placed;

/**
 * Show every file `placed` points into, the first label's file first. Each is introduced
 * by its name and a line and column: the first label's in the first file, and the first
 * shown in any other.
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
        /* The first file is introduced where the primary label is, others where they start. */
        let head = match i {
            0 => here.first(),
            _ => here.iter().min_by_key(|p| (p.line, p.col)),
        };
        let Some(head) = head else {
            continue;
        };
        if i > 0 {
            g.rule(out);
        }
        let arrow = if i == 0 { "-->" } else { ":::" };
        let name = visible(&head.file.name);
        let at = format!("{}{arrow} {name}:{}:{}\n", g.pad, head.line, head.col);
        out.push_str(&at);
        g.rule(out);
        push_file(out, here, g);
    }
}
