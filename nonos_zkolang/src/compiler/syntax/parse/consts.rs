/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Constant declarations and type aliases. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{ConstDecl, TypeAliasDecl};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn const_decl(&mut self) -> PResult<ConstDecl> {
        self.bump();
        let name = self.ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.ty()?;
        self.expect(TokenKind::Eq)?;
        let value = self.expr()?;
        self.expect(TokenKind::Semi)?;
        Ok(ConstDecl { name, ty, value })
    }

    pub(super) fn type_alias(&mut self) -> PResult<TypeAliasDecl> {
        self.bump();
        let name = self.ident()?;
        let generics = self.generic_params()?;
        self.expect(TokenKind::Eq)?;
        let ty = self.ty()?;
        self.expect(TokenKind::Semi)?;
        Ok(TypeAliasDecl { name, generics, ty })
    }
}
