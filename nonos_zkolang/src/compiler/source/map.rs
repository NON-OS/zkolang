/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The source map owns every file one compilation reads and hands out their ids. */

use alloc::string::String;
use alloc::vec::Vec;

use super::file::SourceFile;
use super::span::{FileId, Span};

/** All source files of one compilation. */
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    /** An empty map. */
    pub fn new() -> SourceMap {
        SourceMap { files: Vec::new() }
    }

    /** Add a file and return its id. */
    pub fn add(&mut self, name: String, text: String) -> FileId {
        let id = FileId(u32::try_from(self.files.len()).unwrap_or(u32::MAX - 1));
        self.files.push(SourceFile::new(id, name, text));
        id
    }

    /** The file with this id. */
    pub fn file(&self, id: FileId) -> Option<&SourceFile> {
        self.files.get(id.0 as usize)
    }

    /** Every file, in the order added. */
    pub fn files(&self) -> &[SourceFile] {
        &self.files
    }

    /** The text a span covers. */
    pub fn snippet(&self, span: Span) -> &str {
        self.file(span.file)
            .map(|f| f.slice(span.lo, span.hi))
            .unwrap_or("")
    }

    /** The file name, line and column a span starts at. */
    pub fn locate(&self, span: Span) -> Option<(&str, usize, usize)> {
        let f = self.file(span.file)?;
        let (line, col) = f.line_col(span.lo);
        Some((f.name.as_str(), line, col))
    }
}
