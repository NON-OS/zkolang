/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Generic parameter lists on functions, structs, enums, aliases and impl blocks. */

use alloc::vec::Vec;

use super::super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::GenericParam;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Generic parameters, `<T, const N: usize>`, when present. */
    pub(in crate::compiler::syntax::parse) fn generic_params(
        &mut self,
    ) -> PResult<Vec<GenericParam>> {
        let mut params = Vec::new();
        let open = self.span();
        if !self.eat(TokenKind::Lt) {
            return Ok(params);
        }
        if self.at_generic_close() && !self.after_stray() {
            self.empty_generics(open.to(self.span()));
        }
        while !self.at_generic_close() {
            match self.generic_param() {
                Ok(p) => params.push(p),
                Err(e) if !self.skip_to_generic_close() => return Err(e),
                Err(_) => break,
            }
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        if !self.at_generic_close() {
            let e = self.generic_list_end(false);
            if !self.skip_to_generic_close() {
                return Err(e);
            }
        }
        self.close_generics()?;
        Ok(params)
    }

    /** `T`, or `const N: T`. */
    fn generic_param(&mut self) -> PResult<GenericParam> {
        if !self.eat_kw(Keyword::Const) {
            return Ok(GenericParam::Type(self.ident()?));
        }
        let name = self.ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.ty()?;
        Ok(GenericParam::Const { name, ty })
    }

    /** Report an empty generic list `<>` at `at`. */
    pub(in crate::compiler::syntax::parse) fn empty_generics(
        &mut self,
        at: crate::compiler::source::Span,
    ) {
        let d = Diagnostic::error(Code::UNEXPECTED_TOKEN, "an empty generic list", at, "`<>`")
            .with_help("leave out the `<>`");
        self.diags.push(d);
    }
}
