/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! How the characters of a snippet's window are shown, and where a cut window starts. */

use alloc::format;
use alloc::string::String;

use super::display::cell;
use super::render_window::{CHARS, COLUMNS};

/** Columns shown before the first mark of a cut line. */
const LEAD: usize = 40;

/**
 * Append how `c` is shown and return its columns. A character of no width that is the
 * subject of a label is shown as `<U+XXXX>`.
 */
pub(super) fn shown_cell(c: char, subject: bool, out: &mut String) -> usize {
    let start = out.len();
    let w = cell(c, out);
    if w > 0 || !subject {
        return w;
    }
    out.truncate(start);
    let code = format!("<U+{:04X}>", c as u32);
    out.push_str(&code);
    code.len()
}

/** Whether `line` fits in one window, looking at no more of it than a window holds. */
pub(super) fn whole(line: &str, subjects: &[usize]) -> bool {
    let mut s = String::new();
    let mut used = 0;
    for (n, (i, c)) in line.char_indices().enumerate() {
        s.clear();
        used += shown_cell(c, subjects.contains(&i), &mut s);
        if n >= CHARS || used > COLUMNS {
            return false;
        }
    }
    true
}

/** Where a cut window of `line` starts: at most `LEAD` columns and `CHARS` characters before `at`. */
pub(super) fn lead_start(line: &str, at: usize, subjects: &[usize]) -> usize {
    let (mut from, mut lead) = (at, 0);
    let mut s = String::new();
    for (i, c) in line
        .get(..at)
        .unwrap_or("")
        .char_indices()
        .rev()
        .take(CHARS)
    {
        s.clear();
        lead += shown_cell(c, subjects.contains(&i), &mut s);
        if lead > LEAD {
            break;
        }
        from = i;
    }
    from
}
