/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Generic parameter lists on functions, structs, enums, aliases and impl blocks. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::GenericParam;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Generic parameters, `<T, const N: usize>`, when present. */
    pub(super) fn generic_params(&mut self) -> PResult<Vec<GenericParam>> {
        let mut params = Vec::new();
        if !self.eat(TokenKind::Lt) {
            return Ok(params);
        }
        while !self.at(TokenKind::Gt) {
            if self.eat_kw(Keyword::Const) {
                let name = self.ident()?;
                self.expect(TokenKind::Colon)?;
                let ty = self.ty()?;
                params.push(GenericParam::Const { name, ty });
            } else {
                params.push(GenericParam::Type(self.ident()?));
            }
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Gt)?;
        Ok(params)
    }
}
