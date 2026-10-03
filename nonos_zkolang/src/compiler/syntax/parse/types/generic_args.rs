/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Generic argument lists, `<arg, ...>`. */

use alloc::vec::Vec;

use super::super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::GenericArg;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `<arg, ...>`, the opening `<` current. */
    pub(in crate::compiler::syntax::parse) fn generic_args(&mut self) -> PResult<Vec<GenericArg>> {
        let open = self.expect(TokenKind::Lt)?;
        if self.at_generic_close() && !self.after_stray() {
            self.empty_generics(open.span.to(self.span()));
        }
        /* Each argument is a type or constant, which spends its own nesting level. */
        let mut args = Vec::new();
        while !self.at_generic_close() {
            match self.generic_arg() {
                Ok(a) => args.push(a),
                Err(e) if !self.skip_to_generic_close() => return Err(e),
                Err(_) => break,
            }
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        if !self.at_generic_close() {
            let e = self.generic_list_end(true);
            if !self.skip_to_generic_close() {
                return Err(e);
            }
        }
        self.close_generics()?;
        Ok(args)
    }

    fn generic_arg(&mut self) -> PResult<GenericArg> {
        if self.at(TokenKind::Int) || self.at(TokenKind::LBrace) || self.at(TokenKind::Minus) {
            return Ok(GenericArg::Const(self.const_arg()?));
        }
        Ok(GenericArg::Type(self.ty()?))
    }
}
