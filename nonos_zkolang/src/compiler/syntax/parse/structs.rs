/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Struct declarations and named fields. */

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
        let fields = self.decl_list(TokenKind::RBrace, |p| {
            let start = p.span();
            let (doc, attrs) = p.doc_and_attrs()?;
            let vis = p.field_vis(in_variant);
            let name = p.ident()?;
            p.expect(TokenKind::Colon)?;
            let ty = p.ty()?;
            Ok(FieldDecl {
                attrs,
                vis,
                doc,
                name: Some(name),
                ty,
                span: start.to(p.prev_span()),
            })
        })?;
        Ok(Fields::Named(fields))
    }
}
