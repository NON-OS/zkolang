/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Line lookups on a source file: the line and column of a byte offset, found by binary
 * search over the line table, and the text of one line.
 */

use super::file::SourceFile;

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
        let start = start.min(offset);
        let col = self
            .text
            .get(start..offset)
            .map(|s| s.chars().count())
            .unwrap_or(offset - start);
        (line + 1, col + 1)
    }

    /** The text of a one-based line, without its line terminator. */
    pub fn line_text(&self, line: usize) -> &str {
        let Some(&start) = self.line_starts.get(line.wrapping_sub(1)) else {
            return "";
        };
        let start = start as usize;
        let end = self
            .line_starts
            .get(line)
            .map(|&e| e as usize)
            .unwrap_or(self.text.len());
        let s = self.text.get(start..end).unwrap_or("");
        s.trim_end_matches(['\n', '\r'])
    }
}
