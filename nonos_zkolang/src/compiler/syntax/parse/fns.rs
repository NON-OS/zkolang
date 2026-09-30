/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Function declarations and their parameters. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::FnDecl;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn fn_decl(&mut self, is_const: bool) -> PResult<FnDecl> {
        self.bump();
        let name = self.ident()?;
        let generics = self.generic_params()?;
        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        while !self.at(TokenKind::RParen) {
            params.push(self.param()?);
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect_list_end(TokenKind::RParen)?;
        let ret = if self.eat(TokenKind::Arrow) {
            Some(self.ty()?)
        } else {
            None
        };
        let body = self.block()?;
        Ok(FnDecl {
            is_const,
            name,
            generics,
            params,
            ret,
            body,
        })
    }
}
