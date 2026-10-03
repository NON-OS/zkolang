/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Line lookups on a source file: the line and column of a byte offset, found by binary
 * search over the line table, and the text of one line. A byte-order mark at the start of
 * the file is not part of line 1 (spec section 2).
 */

use super::file::SourceFile;
use super::line_table::{starts_in, MARK_STRIDE};

/** The byte length of a byte-order mark in UTF-8. */
const BOM_LEN: usize = 3;

impl SourceFile {
    /**
     * The one-based line and column of a byte offset. The column counts characters, not
     * bytes, so it matches what an editor shows; an offset past the end clamps to it.
     */
    pub fn line_col(&self, offset: u32) -> (usize, usize) {
        let offset = (offset as usize).min(self.text.len());
        let line = match self.line_starts.binary_search(&(offset as u32)) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let start = self.line_starts.get(line).copied().unwrap_or(0) as usize;
        let start = (start + self.bom_before(start)).min(offset);
        let col = self
            .chars_before(offset)
            .saturating_sub(self.chars_before(start));
        (line + 1, col as usize + 1)
    }

    /** The text of a one-based line, without its line terminator or a leading mark. */
    pub fn line_text(&self, line: usize) -> &str {
        let start = self.line_offset(line) as usize;
        let end = self
            .line_starts
            .get(line)
            .map_or(self.text.len(), |&e| e as usize);
        let s = self.text.get(start..end.max(start)).unwrap_or("");
        s.trim_end_matches(['\n', '\r'])
    }

    /** The byte offset `line_text(line)` starts at; the end of the file past the last line. */
    pub fn line_offset(&self, line: usize) -> u32 {
        let Some(&start) = self.line_starts.get(line.wrapping_sub(1)) else {
            return u32::try_from(self.text.len()).unwrap_or(u32::MAX);
        };
        let start = (start as usize + self.bom_before(start as usize)).min(self.text.len());
        u32::try_from(start).unwrap_or(u32::MAX)
    }

    /** The length of the byte-order mark a line starting at `start` begins with. */
    fn bom_before(&self, start: usize) -> usize {
        if start == 0 && self.text.starts_with('\u{feff}') {
            BOM_LEN
        } else {
            0
        }
    }

    /** The number of characters before byte `offset`, which is at most the text length. */
    fn chars_before(&self, offset: usize) -> u32 {
        let Some(&mark) = self.char_marks.get(offset / MARK_STRIDE) else {
            return u32::try_from(offset).unwrap_or(u32::MAX);
        };
        let from = offset / MARK_STRIDE * MARK_STRIDE;
        let tail = self.text.as_bytes().get(from..offset).unwrap_or(&[]);
        mark.saturating_add(starts_in(tail))
    }
}
