/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Patterns with alternatives, `p | q`, and without, as `let` and parameters take. */

use super::super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{PatKind, Pattern};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A pattern, including alternatives `p | q`. */
    pub(in crate::compiler::syntax::parse) fn pattern(&mut self) -> PResult<Pattern> {
        self.nested(|p| {
            let start = p.span();
            let first = p.pattern_one()?;
            if !p.at(TokenKind::Pipe) {
                return Ok(first);
            }
            let mut alts = alloc::vec![first];
            while p.eat(TokenKind::Pipe) {
                alts.push(p.pattern_one()?);
            }
            let span = start.to(p.prev_span());
            Ok(Pattern {
                id: p.id(),
                kind: PatKind::Or(alts),
                span,
            })
        })
    }

    /** A pattern without top-level alternatives, as a `let` or a parameter takes. */
    pub(in crate::compiler::syntax::parse) fn pattern_no_alt(&mut self) -> PResult<Pattern> {
        self.nested(|p| p.pattern_one())
    }
}
