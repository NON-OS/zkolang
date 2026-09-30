/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Enum declarations and their variants. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{EnumDecl, Fields, Variant};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn enum_decl(&mut self) -> PResult<EnumDecl> {
        self.bump();
        let name = self.ident()?;
        let generics = self.generic_params()?;
        self.expect(TokenKind::LBrace)?;
        let mut variants = Vec::new();
        while !self.at(TokenKind::RBrace) {
            let start = self.span();
            let (doc, attrs) = self.doc_and_attrs()?;
            let vname = self.ident()?;
            let fields = match self.kind() {
                TokenKind::LParen => self.tuple_fields()?,
                TokenKind::LBrace => self.named_fields()?,
                _ => Fields::Unit,
            };
            variants.push(Variant {
                id: self.id(),
                attrs,
                doc,
                name: vname,
                fields,
                span: start.to(self.prev_span()),
            });
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(EnumDecl {
            name,
            generics,
            variants,
        })
    }
}
