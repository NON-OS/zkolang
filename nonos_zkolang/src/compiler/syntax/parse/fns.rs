/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Function declarations and their parameters. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{FnDecl, Param};
use crate::compiler::syntax::keyword::Keyword;
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
        self.expect(TokenKind::RParen)?;
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

    fn param(&mut self) -> PResult<Param> {
        let start = self.span();
        if self.at_kw(Keyword::SelfValue) {
            self.bump();
            return Ok(Param::SelfParam {
                by_ref_mut: false,
                span: start,
            });
        }
        if self.at(TokenKind::Amp)
            && self.peek(1) == TokenKind::Kw(Keyword::Mut)
            && self.peek(2) == TokenKind::Kw(Keyword::SelfValue)
        {
            self.bump();
            self.bump();
            self.bump();
            return Ok(Param::SelfParam {
                by_ref_mut: true,
                span: start.to(self.prev_span()),
            });
        }
        let pat = self.pattern_no_alt()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.ty()?;
        Ok(Param::Typed { pat, ty })
    }
}
