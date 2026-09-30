/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The part of a source line a snippet shows. A line that fits is shown whole; a longer one
 * is cut to a window, with `...` where it is cut, that starts at most `LEAD` columns before
 * the first mark and holds at most `COLUMNS` columns and `CHARS` characters. So a snippet
 * on a long line costs what one on a short line does, and the marks it draws fall in it.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::display::floor;
use super::render_cells::{lead_start, shown_cell, whole};

/** Columns a cut line is shown in. */
pub(super) const COLUMNS: usize = 100;
/** Characters a window holds, however narrow they are. */
pub(super) const CHARS: usize = 400;
/** What stands for the text a cut removed. */
const CUT: &str = "...";

/** The shown part of one line. */
pub(super) struct Window {
    /** Each shown character's byte offset and display column, then the window's end. */
    cols: Vec<(usize, usize)>,
    pub(super) shown: String,
}

impl Window {
    /**
     * The window of `line` that shows byte `at` and what follows it. A character of no
     * width that a label starts at, one of `subjects`, is shown as `<U+XXXX>`, so that
     * its mark has something to stand under.
     */
    pub(super) fn around(line: &str, at: usize, subjects: &[usize]) -> Window {
        let at = floor(line, at.min(line.len()));
        let whole = whole(line, subjects);
        let from = if whole {
            0
        } else {
            lead_start(line, at, subjects)
        };
        let mut shown = String::from(if from > 0 { CUT } else { "" });
        let (mut used, mut cols, mut to) = (shown.len(), Vec::new(), from);
        let mut s = String::new();
        for (i, c) in line[from..].char_indices().take(CHARS) {
            s.clear();
            let w = shown_cell(c, subjects.contains(&(from + i)), &mut s);
            if used + w > COLUMNS && !whole {
                break;
            }
            cols.push((from + i, used));
            shown.push_str(&s);
            used += w;
            to = from + i + c.len_utf8();
        }
        cols.push((to, used));
        if to < line.len() {
            shown.push_str(CUT);
        }
        Window { cols, shown }
    }

    /** The display column byte `at` of the line is shown at, clamped to the window. */
    pub(super) fn col(&self, at: usize) -> usize {
        let i = self.cols.partition_point(|&(b, _)| b <= at);
        self.cols.get(i.saturating_sub(1)).map_or(0, |&(_, c)| c)
    }
}
