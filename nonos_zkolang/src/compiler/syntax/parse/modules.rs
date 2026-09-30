/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Module declarations: `mod name;` for a file, or an inline body in braces. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::ModDecl;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn mod_decl(&mut self) -> PResult<ModDecl> {
        self.bump();
        let name = self.ident()?;
        if self.eat(TokenKind::Semi) {
            return Ok(ModDecl {
                name,
                body: None,
                inner_attrs: Vec::new(),
                inner_doc: None,
            });
        }
        let open = self.expect(TokenKind::LBrace)?;
        let (inner_doc, inner_attrs) = self.inner_doc_and_attrs(open.span.hi)?;
        let items = self.nested(|p| Ok(p.items(true)))?;
        if !self.at(TokenKind::RBrace) {
            self.diags.push(Diagnostic::error(
                Code::UNCLOSED_DELIMITER,
                "unclosed module",
                open.span,
                "this `{` is never closed",
            ));
            return Err(super::parser::Reported);
        }
        self.bump();
        Ok(ModDecl {
            name,
            body: Some(items),
            inner_attrs,
            inner_doc,
        })
    }
}
