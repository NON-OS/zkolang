/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One source file: its display name, its text, and the tables that convert a byte offset
 * to a line and column without rescanning the line.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::line_table::{char_marks, line_starts};
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
    /** The characters before every `MARK_STRIDE`th byte; empty for an ASCII file. */
    pub(super) char_marks: Vec<u32>,
}

impl SourceFile {
    /** A file with its line table built. */
    pub fn new(id: FileId, name: String, text: String) -> SourceFile {
        SourceFile {
            id,
            name,
            line_starts: line_starts(text.as_bytes()),
            char_marks: char_marks(text.as_bytes()),
            text,
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
