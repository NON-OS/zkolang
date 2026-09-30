/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Enum declarations and their variants. */

use super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{EnumDecl, Fields, Variant};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn enum_decl(&mut self) -> PResult<EnumDecl> {
        self.bump();
        let name = self.ident()?;
        let generics = self.generic_params()?;
        self.expect(TokenKind::LBrace)?;
        let variants = self.decl_list(TokenKind::RBrace, |p| p.variant())?;
        Ok(EnumDecl {
            name,
            generics,
            variants,
        })
    }

    /** A variant: its name, and its fields if it has any. */
    fn variant(&mut self) -> PResult<Variant> {
        let start = self.span();
        let (doc, attrs) = self.doc_and_attrs()?;
        let name = self.ident()?;
        let fields = match self.kind() {
            TokenKind::LParen => self.tuple_fields(true)?,
            TokenKind::LBrace => self.named_fields(true)?,
            TokenKind::Eq => {
                self.variant_value();
                Fields::Unit
            }
            _ => Fields::Unit,
        };
        Ok(Variant {
            id: self.id(),
            attrs,
            doc,
            name,
            fields,
            span: start.to(self.prev_span()),
        })
    }

    /** Report and skip `= value` after a variant: a variant has no value of its own. */
    fn variant_value(&mut self) {
        let start = self.bump().span;
        self.skip_elem(TokenKind::RBrace);
        let d = Diagnostic::error(
            Code::UNEXPECTED_TOKEN,
            "a variant takes no value",
            start.to(self.prev_span()),
            "a value for a variant",
        )
        .with_help("a variant is told apart by its position; match on it to get a number");
        self.diags.push(d);
    }
}
