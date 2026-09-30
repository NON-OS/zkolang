/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Struct declarations and named fields. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{FieldDecl, Fields, StructDecl};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn struct_decl(&mut self) -> PResult<StructDecl> {
        self.bump();
        let name = self.ident()?;
        let generics = self.generic_params()?;
        let fields = match self.kind() {
            TokenKind::LBrace => self.named_fields(false)?,
            TokenKind::LParen => {
                let f = self.tuple_fields(false)?;
                self.expect(TokenKind::Semi)?;
                f
            }
            TokenKind::Semi => {
                self.bump();
                Fields::Unit
            }
            _ => return Err(self.unexpected("`{`, `(` or `;`")),
        };
        Ok(StructDecl {
            name,
            generics,
            fields,
        })
    }

    /** `{ a: T, pub b: U }`; in a variant, whose fields take no `pub`, `{ a: T }`. */
    pub(super) fn named_fields(&mut self, in_variant: bool) -> PResult<Fields> {
        self.expect(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        while !self.at(TokenKind::RBrace) {
            let start = self.span();
            let (doc, attrs) = self.doc_and_attrs()?;
            let vis = self.field_vis(in_variant);
            let name = self.ident()?;
            self.expect(TokenKind::Colon)?;
            let ty = self.ty()?;
            fields.push(FieldDecl {
                attrs,
                vis,
                doc,
                name: Some(name),
                ty,
                span: start.to(self.prev_span()),
            });
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect_list_end(TokenKind::RBrace)?;
        Ok(Fields::Named(fields))
    }
}
