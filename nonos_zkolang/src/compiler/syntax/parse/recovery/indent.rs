/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Indentation, which tells where a block the source left open was meant to end. */

use super::super::parser::Parser;

impl<'a> Parser<'a> {
    /** The spaces and tabs before `at` on its line, if nothing else stands before it there. */
    pub(in crate::compiler::syntax::parse) fn indent_before(&self, at: u32) -> Option<usize> {
        let b = self.text.as_bytes();
        let mut i = at as usize;
        let mut n = 0;
        while let Some(&c) = i.checked_sub(1).and_then(|j| b.get(j)) {
            match c {
                b' ' | b'\t' => n += 1,
                b'\n' | b'\r' => return Some(n),
                _ => return None,
            }
            i -= 1;
        }
        Some(n)
    }

    /** The indentation of the line that holds `at`. */
    pub(in crate::compiler::syntax::parse) fn line_indent(&self, at: u32) -> usize {
        let start = self.layout.line_start(at) as usize;
        let line = self.text.as_bytes().get(start..).unwrap_or(&[]);
        line.iter()
            .take_while(|&&c| c == b' ' || c == b'\t')
            .count()
    }
}
