/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A literal pattern: an integer, possibly negative, or a boolean. */

use super::super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Lit, PatKind, Pattern};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::lex::{int_literal, IntLit};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A literal pattern: an integer, possibly negative, or `true` or `false` if `bools`. */
    pub(in crate::compiler::syntax::parse) fn literal_pattern(&mut self, bools: bool) -> PResult<Pattern> {
        let start = self.span();
        let negative = self.eat(TokenKind::Minus);
        let lit = match self.kind() {
            TokenKind::Int => {
                let t = self.bump();
                let l = int_literal(self.text_of(t)).unwrap_or(IntLit {
                    value: 0,
                    suffix: None,
                });
                Lit::Int {
                    value: l.value,
                    suffix: l.suffix,
                    span: t.span,
                }
            }
            TokenKind::Kw(Keyword::True) | TokenKind::Kw(Keyword::False) if bools && !negative => {
                let t = self.bump();
                Lit::Bool {
                    value: t.kind == TokenKind::Kw(Keyword::True),
                    span: t.span,
                }
            }
            _ => return Err(self.unexpected("an integer literal")),
        };
        let span = start.to(self.prev_span());
        Ok(Pattern {
            id: self.id(),
            kind: PatKind::Lit { lit, negative },
            span,
        })
    }
}
