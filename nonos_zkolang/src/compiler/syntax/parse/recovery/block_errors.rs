/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Errors inside a block: a block that is never closed, and an item written inside one. */

use super::super::parser::{Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Report a block whose `{` at `open` is never closed. */
    pub(in crate::compiler::syntax::parse) fn block_unclosed(&mut self, open: Span) -> Reported {
        self.diags.push(Diagnostic::error(
            Code::UNCLOSED_DELIMITER,
            "unclosed block",
            open,
            "this `{` is never closed",
        ));
        Reported
    }

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
        self.recover_item_in_block();
    }

    /** Skip an item written inside a block, braces and all. */
    fn recover_item_in_block(&mut self) {
        let mut depth: usize = 0;
        loop {
            match self.kind() {
                TokenKind::Eof => return,
                TokenKind::LBrace | TokenKind::LParen | TokenKind::LBracket => depth += 1,
                TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket => {
                    if depth == 0 {
                        return;
                    }
                    depth -= 1;
                    if depth == 0 && self.at(TokenKind::RBrace) {
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
