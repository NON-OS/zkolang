/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Literal patterns, and the ranges `lo..=hi` between two of them. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Lit, PatKind, Pattern};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::lex::{int_literal, IntLit};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A literal pattern, or a range `lo..=hi` of two, starting at `start`. */
    pub(super) fn pattern_lit_or_range(&mut self, start: Span) -> PResult<Pattern> {
        let lo = self.literal_pattern()?;
        if !self.eat(TokenKind::DotDotEq) {
            return Ok(lo);
        }
        let hi = self.literal_pattern()?;
        let kind = PatKind::Range {
            lo: Box::new(lo),
            hi: Box::new(hi),
        };
        let span = start.to(self.prev_span());
        Ok(Pattern {
            id: self.id(),
            kind,
            span,
        })
    }

    /** A literal pattern: an integer, possibly negative, or `true` or `false`. */
    fn literal_pattern(&mut self) -> PResult<Pattern> {
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
            TokenKind::Kw(Keyword::True) | TokenKind::Kw(Keyword::False) if !negative => {
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
