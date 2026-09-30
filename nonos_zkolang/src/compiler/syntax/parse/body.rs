/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The body of an `if`, `for` or `while`, recovered when its `{` is missing. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::Block;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * The block of an `if`, `for` or `while`. When its `{` is missing and a `}` ends the
     * line, the statements before that `}` are taken as the body, so the `}` does not
     * close the block around them.
     */
    pub(super) fn body_block(&mut self) -> PResult<Block> {
        if self.at(TokenKind::LBrace) {
            return self.block();
        }
        let missing = self.unexpected("`{`");
        if !self.closer_on_line(TokenKind::RBrace) {
            return Err(missing);
        }
        let open = self.span();
        let saved = self.no_struct;
        self.no_struct = false;
        let body = self.nested(|p| p.block_body(open));
        self.no_struct = saved;
        body
    }
}
