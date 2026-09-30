/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The part of a source line a snippet shows. A short line is shown whole; a long one is
 * cut to a window around the marked text, with `...` where it is cut, so a diagnostic on
 * a long line costs the same as one on a short line.
 */

use alloc::string::String;

use super::display::{cell, columns, floor};

/** Characters shown before the first mark of a cut line. */
const BEFORE: usize = 40;
/** Columns a cut line is shown in. */
const COLUMNS: usize = 100;
/** What stands for the text a cut removed. */
const CUT: &str = "...";

/** The shown part of one line: bytes `from..to` of it, as rendered. */
pub(super) struct Window<'a> {
    line: &'a str,
    from: usize,
    to: usize,
    pub(super) shown: String,
}

impl<'a> Window<'a> {
    /** The window of `line` that shows byte `at` and as much after it as fits. */
    pub(super) fn around(line: &'a str, at: usize) -> Window<'a> {
        let at = floor(line, at.min(line.len()));
        if line.len() <= COLUMNS {
            return Window::span(line, 0, line.len());
        }
        let from = line[..at].char_indices().rev().take(BEFORE).last();
        let from = from.map_or(at, |(i, _)| i);
        let mut used = if from > 0 { CUT.len() } else { 0 };
        let mut to = from;
        for (i, c) in line[from..].char_indices() {
            used += columns(c.encode_utf8(&mut [0; 4]));
            if used > COLUMNS {
                break;
            }
            to = from + i + c.len_utf8();
        }
        Window::span(line, from, to)
    }

    fn span(line: &'a str, from: usize, to: usize) -> Window<'a> {
        let mut shown = String::from(if from > 0 { CUT } else { "" });
        line[from..to].chars().for_each(|c| {
            cell(c, &mut shown);
        });
        if to < line.len() {
            shown.push_str(CUT);
        }
        Window {
            line,
            from,
            to,
            shown,
        }
    }

    /** The display column byte `at` of the line is shown at, clamped to the window. */
    pub(super) fn col(&self, at: usize) -> usize {
        let at = floor(self.line, at.clamp(self.from, self.to));
        let cut = if self.from > 0 { CUT.len() } else { 0 };
        cut + columns(&self.line[self.from..at])
    }
}
