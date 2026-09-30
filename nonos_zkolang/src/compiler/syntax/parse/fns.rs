/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Function declarations and their parameters. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use super::recovery::starts_item;
use crate::compiler::syntax::ast::{FnDecl, GenericParam, Ident, Param, Type};
use crate::compiler::syntax::token::TokenKind;

/** A function's name, generic parameters, parameters and result type. */
type Signature = (Ident, Vec<GenericParam>, Vec<Param>, Option<Type>);

impl<'a> Parser<'a> {
    pub(super) fn fn_decl(&mut self, is_const: bool) -> PResult<FnDecl> {
        self.bump();
        let (name, generics, params, ret) = match self.fn_signature() {
            Ok(sig) => sig,
            Err(e) => {
                /* The body's own errors are independent of the signature's: report them. */
                self.body_after_bad_signature();
                return Err(e);
            }
        };
        if !self.at(TokenKind::LBrace) {
            let expected = if ret.is_some() { "`{`" } else { "`->` or `{`" };
            let e = self.report_unexpected(expected, true);
            /* An item after the signature is the next one, not this one's body. */
            if !starts_item(self.kind()) {
                self.body_after_bad_signature();
            }
            return Err(e);
        }
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

    /** A function's name, generic parameters, parameters and result type. */
    fn fn_signature(&mut self) -> PResult<Signature> {
        let name = self.ident()?;
        let generics = self.generic_params()?;
        self.expect(TokenKind::LParen)?;
        let params = self.decl_list(TokenKind::RParen, |p| p.param())?;
        let ret = if self.eat(TokenKind::Arrow) {
            Some(self.ty()?)
        } else {
            None
        };
        Ok((name, generics, params, ret))
    }
}
