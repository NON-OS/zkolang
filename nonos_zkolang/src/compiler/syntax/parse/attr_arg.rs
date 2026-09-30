/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * An argument in an attribute's parentheses: a name, which may be a keyword such as
 * `declassify`, with an optional `= literal`; or a literal.
 */

use alloc::string::String;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{AttrArg, Ident};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** One argument in an attribute's parentheses. */
    pub(super) fn attr_arg(&mut self) -> PResult<AttrArg> {
        let t = self.tok();
        let named = match t.kind {
            TokenKind::Kw(Keyword::True | Keyword::False) => false,
            TokenKind::Ident | TokenKind::Kw(_) => true,
            TokenKind::Str | TokenKind::Int => false,
            _ => return Err(self.unexpected("a name or a literal")),
        };
        if !named {
            return Ok(AttrArg::Lit(self.attr_lit()?));
        }
        self.bump();
        let name = Ident {
            name: String::from(self.text_of(t)),
            span: t.span,
        };
        let value = if self.eat(TokenKind::Eq) {
            Some(self.attr_lit()?)
        } else {
            None
        };
        Ok(AttrArg::Named { name, value })
    }
}
