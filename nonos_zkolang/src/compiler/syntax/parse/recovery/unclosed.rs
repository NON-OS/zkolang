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

use super::super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Whether the current token begins an item that the open block cannot hold: an item
     * on its own line at or left of the indentation of the item being parsed, in a file
     * with a `}` missing. In an impl block, `fns_belong` keeps a function in the block.
     */
    pub(in crate::compiler::syntax::parse) fn at_outer_item(&self, fns_belong: bool) -> bool {
        if !self.layout.missing_closers {
            return false;
        }
        let Some((k, next)) = self.item_keyword_ahead() else {
            return false;
        };
        let is_fn = k == TokenKind::Kw(Keyword::Fn) || next == TokenKind::Kw(Keyword::Fn);
        let indent = self.indent_before(self.span().lo);
        !(fns_belong && is_fn) && indent.is_some_and(|i| i <= self.item_indent)
    }

    /** Report, once, that the `what` opened at `open` is never closed. */
    pub(in crate::compiler::syntax::parse) fn report_unclosed(&mut self, open: Span, what: &str) {
        if self.unclosed_reported {
            return;
        }
        self.unclosed_reported = true;
        if self.left_open_since(open) {
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
}
