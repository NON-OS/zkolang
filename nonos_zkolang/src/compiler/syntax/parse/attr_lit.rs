/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The literal an attribute takes as its value or argument: a string, an integer, or a bool. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::Lit;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::lex::{int_literal, str_literal};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn attr_lit(&mut self) -> PResult<Lit> {
        let t = self.tok();
        match t.kind {
            TokenKind::Str => {
                self.bump();
                Ok(Lit::Str {
                    value: str_literal(self.text_of(t)),
                    span: t.span,
                })
            }
            TokenKind::Int => {
                self.bump();
                let l = int_literal(self.text_of(t))
                    .map(|l| (l.value, l.suffix))
                    .unwrap_or((0, None));
                Ok(Lit::Int {
                    value: l.0,
                    suffix: l.1,
                    span: t.span,
                })
            }
            TokenKind::Kw(Keyword::True) | TokenKind::Kw(Keyword::False) => {
                self.bump();
                Ok(Lit::Bool {
                    value: t.kind == TokenKind::Kw(Keyword::True),
                    span: t.span,
                })
            }
            _ => Err(self.unexpected("a literal")),
        }
    }
}
