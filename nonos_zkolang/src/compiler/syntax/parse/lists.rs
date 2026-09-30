/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Comma-separated lists of declarations: parameters, fields and variants. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * The elements up to `close`, which is consumed, each parsed by `elem`. An error in
     * one element is reported and the parse goes on at the next, so each is reported. A
     * missing `,` at the end of a line is reported, and the next line read as the next
     * element.
     */
    pub(super) fn decl_list<T>(
        &mut self,
        close: TokenKind,
        mut elem: impl FnMut(&mut Self) -> PResult<T>,
    ) -> PResult<Vec<T>> {
        let mut out = Vec::new();
        let mut failed = false;
        while !self.at(close) {
            let from = self.pos;
            match elem(self) {
                Ok(e) => out.push(e),
                Err(_) if self.recover_elem(from, close) => {
                    failed = true;
                    continue;
                }
                Err(e) => return Err(e),
            }
            if self.eat(TokenKind::Comma) || self.at(close) {
                continue;
            }
            /* A list gone wrong that ends at another closer: the same mistake, told once. */
            if failed
                && matches!(
                    self.kind(),
                    TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace
                )
            {
                return Err(Reported);
            }
            let Err(e) = self.expect_list_end(close) else {
                continue;
            };
            let next_line = self.line_end_before(self.span()).is_some();
            let ends = matches!(self.kind(), TokenKind::LBrace | TokenKind::Semi);
            if next_line && !ends && !self.item_starts_line() {
                continue;
            }
            if !self.recover_elem(self.pos, close) {
                return Err(e);
            }
        }
        self.bump();
        Ok(out)
    }
}
