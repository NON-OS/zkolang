/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One source file: its display name, its text, and the byte offset each line starts at,
 * so a span converts to a line and column in logarithmic time.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::span::FileId;

/** A source file held by the source map. */
#[derive(Clone, Debug)]
pub struct SourceFile {
    pub id: FileId,
    /** The name a diagnostic shows, a path relative to the package root. */
    pub name: String,
    pub text: String,
    /** The byte offset of each line's first byte; `file_lines` reads it. */
    pub(super) line_starts: Vec<u32>,
}

impl SourceFile {
    /** A file with its line table built. */
    pub fn new(id: FileId, name: String, text: String) -> SourceFile {
        let mut line_starts = Vec::new();
        line_starts.push(0);
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(u32::try_from(i + 1).unwrap_or(u32::MAX));
            }
        }
        SourceFile {
            id,
            name,
            text,
            line_starts,
        }
    }

    /** The number of lines. */
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /** The source text a span covers, or the empty string for a span outside the file. */
    pub fn slice(&self, lo: u32, hi: u32) -> &str {
        self.text.get(lo as usize..hi as usize).unwrap_or("")
    }
}
