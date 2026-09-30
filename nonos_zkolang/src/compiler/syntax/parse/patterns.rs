/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Patterns. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{PatKind, Pattern};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A pattern, including alternatives `p | q`. */
    pub(super) fn pattern(&mut self) -> PResult<Pattern> {
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
    pub(super) fn pattern_no_alt(&mut self) -> PResult<Pattern> {
        self.nested(|p| p.pattern_one())
    }

    fn pattern_one(&mut self) -> PResult<Pattern> {
        let start = self.span();
        let kind = match self.kind() {
            TokenKind::Wildcard => {
                self.bump();
                PatKind::Wild
            }
            TokenKind::Kw(Keyword::Mut) => {
                self.bump();
                let name = self.ident()?;
                PatKind::Bind {
                    name,
                    mutable: true,
                }
            }
            TokenKind::Int
            | TokenKind::Minus
            | TokenKind::Kw(Keyword::True)
            | TokenKind::Kw(Keyword::False) => return self.pattern_lit_or_range(start),
            TokenKind::LParen => return self.pattern_paren(start),
            TokenKind::LBracket => self.pattern_array()?,
            _ if self.at_path_start() => self.pattern_path()?,
            _ => return Err(self.unexpected("a pattern")),
        };
        let span = start.to(self.prev_span());
        Ok(Pattern {
            id: self.id(),
            kind,
            span,
        })
    }
}
