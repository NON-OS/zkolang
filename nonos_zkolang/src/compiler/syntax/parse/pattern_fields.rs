/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The fields of a struct pattern, with the `..` that ignores the rest. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::FieldPat;
use crate::compiler::syntax::token::TokenKind;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /** The fields of a struct pattern after its `{`, and whether it ends in `..`. */
    pub(super) fn field_patterns(&mut self) -> PResult<(Vec<FieldPat>, bool)> {
        let mut fields = Vec::new();
        let mut rest = false;
        while !self.at(TokenKind::RBrace) {
            if self.eat(TokenKind::DotDot) {
                rest = true;
                self.eat(TokenKind::Comma);
                break;
            }
            let start = self.span();
            let name = self.ident()?;
            let pat = if self.eat(TokenKind::Colon) {
                Some(self.pattern()?)
            } else {
                None
            };
            fields.push(FieldPat {
                name,
                pat,
                span: start.to(self.prev_span()),
            });
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect_list_end(TokenKind::RBrace)?;
        Ok((fields, rest))
    }
}
