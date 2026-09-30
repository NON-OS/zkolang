/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Where a bracket an error left open is taken to end during recovery. */

use super::super::parser::Parser;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Whether `k`, the current token, cannot continue a bracket the error left open: a
     * `let`; a `;` that ends its line, where `[a, b; c]` still goes on to its `]`; or an
     * item that starts a line at or left of the indentation of the item being parsed.
     */
    pub(in crate::compiler::syntax::parse) fn ends_bracket(&self, k: TokenKind) -> bool {
        match k {
            TokenKind::Kw(Keyword::Let) => true,
            TokenKind::Semi => self.ends_line(),
            _ => self.item_starts_line() && self.line_indent(self.span().lo) <= self.item_indent,
        }
    }

    /** Whether the current token is the last on its line. */
    pub(in crate::compiler::syntax::parse) fn ends_line(&self) -> bool {
        let (Some(a), Some(b)) = (self.tokens.get(self.pos), self.tokens.get(self.pos + 1)) else {
            return true;
        };
        let gap = self
            .text
            .get(a.span.hi as usize..b.span.lo as usize)
            .unwrap_or("");
        b.kind == TokenKind::Eof || gap.contains(['\n', '\r'])
    }
}
