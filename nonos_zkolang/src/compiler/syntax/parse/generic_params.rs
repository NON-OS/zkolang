/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Generic parameter lists on functions, structs, enums, aliases and impl blocks. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::GenericParam;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Generic parameters, `<T, const N: usize>`, when present. */
    pub(super) fn generic_params(&mut self) -> PResult<Vec<GenericParam>> {
        let mut params = Vec::new();
        let open = self.span();
        if !self.eat(TokenKind::Lt) {
            return Ok(params);
        }
        if self.at(TokenKind::Gt) {
            self.empty_generics(open.to(self.span()));
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

    /** Report an empty generic list `<>` at `at`. */
    pub(super) fn empty_generics(&mut self, at: crate::compiler::source::Span) {
        let d = Diagnostic::error(Code::UNEXPECTED_TOKEN, "an empty generic list", at, "`<>`")
            .with_help("leave out the `<>`");
        self.diags.push(d);
    }
}
