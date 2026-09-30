/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading a manifest line by line: a header, an entry, or nothing but a comment. */

use alloc::string::String;

use super::toml::Entry;
use crate::compiler::source::{FileId, Span};

/** What one line holds. */
pub(super) enum Line {
    Header(String, Span),
    Entry(Entry),
}

/** A place in a manifest's text. */
pub(super) struct Cursor<'t> {
    pub(super) file: FileId,
    text: &'t [u8],
    at: usize,
}

impl<'t> Cursor<'t> {
    pub(super) fn new(file: FileId, text: &'t str) -> Cursor<'t> {
        Cursor {
            file,
            text: text.as_bytes(),
            at: 0,
        }
    }

    pub(super) fn done(&self) -> bool {
        self.at >= self.text.len()
    }

    pub(super) fn peek(&self) -> Option<u8> {
        self.text.get(self.at).copied()
    }

    pub(super) fn bump(&mut self) {
        self.at = self.at.saturating_add(1);
    }

    pub(super) fn pos(&self) -> u32 {
        u32::try_from(self.at).unwrap_or(u32::MAX)
    }

    pub(super) fn span_from(&self, lo: u32) -> Span {
        Span::new(self.file, lo, self.pos())
    }
}
