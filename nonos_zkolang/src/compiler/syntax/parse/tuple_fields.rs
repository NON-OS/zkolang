/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Positional fields, as in a tuple struct or a tuple variant. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{FieldDecl, Fields, Visibility};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `(T, pub U)` */
    pub(super) fn tuple_fields(&mut self) -> PResult<Fields> {
        self.expect(TokenKind::LParen)?;
        let mut fields = Vec::new();
        while !self.at(TokenKind::RParen) {
            let start = self.span();
            let (doc, attrs) = self.doc_and_attrs()?;
            let vis = if self.eat_kw(Keyword::Pub) {
                Visibility::Public
            } else {
                Visibility::Private
            };
            let ty = self.ty()?;
            fields.push(FieldDecl {
                attrs,
                vis,
                doc,
                name: None,
                ty,
                span: start.to(self.prev_span()),
            });
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect_list_end(TokenKind::RParen)?;
        Ok(Fields::Tuple(fields))
    }
}
