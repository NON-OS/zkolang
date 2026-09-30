/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Errors inside a block: a block that is never closed, and an item written inside one. */

use super::super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Report an item written inside a block, starting at `start`, and skip it. */
    pub(in crate::compiler::syntax::parse) fn stmt_item_in_block(&mut self, start: Span) {
        self.diags.push(
            Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                "items are declared at module level",
                start,
                "an item inside a block",
            )
            .with_help("move the item out of the function body"),
        );
        self.skip_item();
    }

    /**
     * Skip an item where none may stand, braces and all. An item that ends in `;` ends
     * there, not at a `}` inside it, or before the next item that starts a line.
     */
    pub(in crate::compiler::syntax::parse) fn skip_item(&mut self) {
        let ahead = self.item_keyword_at();
        let semi = match ahead {
            Some((_, TokenKind::Kw(Keyword::Const), next)) => next != TokenKind::Kw(Keyword::Fn),
            Some((_, k, _)) => matches!(k, TokenKind::Kw(Keyword::Use | Keyword::Type)),
            None => false,
        };
        /* The item's own keyword, after its attributes, is not the next item. */
        let keyword = ahead.map_or(self.pos, |(i, _, _)| i);
        let mut depth: usize = 0;
        loop {
            match self.kind() {
                TokenKind::Eof => return,
                _ if semi && depth == 0 && self.pos > keyword && self.item_starts_line() => return,
                TokenKind::LBrace | TokenKind::LParen | TokenKind::LBracket => depth += 1,
                TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket => {
                    if depth == 0 {
                        return;
                    }
                    depth -= 1;
                    if depth == 0 && !semi && self.at(TokenKind::RBrace) {
                        self.bump();
                        return;
                    }
                }
                TokenKind::Semi if depth == 0 => {
                    self.bump();
                    return;
                }
                _ => {}
            }
            self.bump();
        }
    }
}
