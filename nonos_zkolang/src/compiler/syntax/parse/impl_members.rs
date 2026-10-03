/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The functions of an impl block, between its braces. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Item;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * The functions of an impl block whose `{` is at `open`, up to its `}`, which is
     * consumed. An item that starts a line left of the block's functions ends a block
     * whose `}` is missing, and is left to the module.
     */
    pub(super) fn impl_members(&mut self, open: Span) -> PResult<Vec<Item>> {
        let mut items = Vec::new();
        while !self.at(TokenKind::RBrace) {
            if self.at(TokenKind::Eof) {
                self.report_unclosed(open, "impl block");
                return Err(Reported);
            }
            if self.at_outer_item(true) {
                self.report_unclosed(open, "impl block");
                return Ok(items);
            }
            if self.at_include() {
                self.skip_include();
                continue;
            }
            if self.at_non_fn_item() {
                continue;
            }
            let before = self.pos;
            match self.impl_item() {
                Ok(item) => items.push(item),
                Err(_) => {
                    self.recover_item(before);
                    if self.pos == before {
                        self.bump();
                    }
                }
            }
        }
        self.bump();
        Ok(items)
    }
}
