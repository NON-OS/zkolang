/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Indentation, which tells where a block the source left open was meant to end. */

use super::super::parser::Parser;

impl<'a> Parser<'a> {
    /** The spaces and tabs before `at` on its line, if nothing else stands before it there. */
    pub(in crate::compiler::syntax::parse) fn indent_before(&self, at: u32) -> Option<usize> {
        let (start, indent) = self.layout.line_of(at);
        (at == start.saturating_add(indent)).then_some(indent as usize)
    }

    /** The indentation of the line that holds `at`. */
    pub(in crate::compiler::syntax::parse) fn line_indent(&self, at: u32) -> usize {
        self.layout.line_of(at).1 as usize
    }
}
