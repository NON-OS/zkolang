/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Bracketed patterns: parentheses, tuples and arrays. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{PatKind, Pattern};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `()`, `(p)` or `(p, q, ...)`, the `(` current and at `start`. */
    pub(super) fn pattern_paren(&mut self, start: Span) -> PResult<Pattern> {
        self.bump();
        let kind = if self.eat(TokenKind::RParen) {
            PatKind::Tuple(Vec::new())
        } else {
            let first = self.pattern()?;
            if self.eat(TokenKind::RParen) {
                return Ok(first);
            }
            let mut elems = alloc::vec![first];
            while self.eat(TokenKind::Comma) {
                if self.at(TokenKind::RParen) {
                    break;
                }
                elems.push(self.pattern()?);
            }
            self.expect(TokenKind::RParen)?;
            PatKind::Tuple(elems)
        };
        let span = start.to(self.prev_span());
        Ok(Pattern {
            id: self.id(),
            kind,
            span,
        })
    }

    /** `[p, q, ...]`, the `[` current. */
    pub(super) fn pattern_array(&mut self) -> PResult<PatKind> {
        self.bump();
        let mut elems = Vec::new();
        while !self.at(TokenKind::RBracket) {
            elems.push(self.pattern()?);
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBracket)?;
        Ok(PatKind::Array(elems))
    }
}
