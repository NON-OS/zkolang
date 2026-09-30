/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Unclosed blocks. A block, impl block or module left open is reported once: where an
 * item begins on its own line at or left of the indentation of the item the block belongs
 * to, which is taken to close it, or at the end of the file. Every construct still open
 * there closes with it, unreported.
 */

use alloc::format;

use super::super::parser::{starts_item, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Whether the current token begins an item that the open block cannot hold. */
    pub(in crate::compiler::syntax::parse) fn at_outer_item(&self) -> bool {
        let k = self.kind();
        let is_item = starts_item(k)
            && !(k == TokenKind::Kw(Keyword::Const) && self.peek(1) != TokenKind::Ident);
        let indent = self.indent_before(self.span().lo);
        is_item && indent.is_some_and(|i| i <= self.item_indent)
    }

    /** Report, once, that the `what` opened at `open` is never closed. */
    pub(in crate::compiler::syntax::parse) fn report_unclosed(&mut self, open: Span, what: &str) {
        if self.unclosed_reported {
            return;
        }
        self.unclosed_reported = true;
        if self.string_left_open_since(open) {
            /* An unterminated string ran over the `}`; its own error explains this one. */
            return;
        }
        let msg = format!("unclosed {what}");
        let d = Diagnostic::error(
            Code::UNCLOSED_DELIMITER,
            msg,
            open,
            "this `{` is never closed",
        );
        let d = if self.at(TokenKind::Eof) {
            d
        } else {
            d.with_label(
                self.span(),
                "an item starts here, so a `}` is missing before it",
            )
        };
        self.diags.push(d);
    }

    /** Whether a string literal after `open` was left unterminated. */
    fn string_left_open_since(&self, open: Span) -> bool {
        let before = self.tokens.get(..self.pos).unwrap_or(&[]);
        before
            .iter()
            .rev()
            .take_while(|t| t.span.lo > open.lo)
            .any(|t| t.kind == TokenKind::Error && self.text_of(*t).starts_with('"'))
    }
}
