/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Patterns. */

use super::super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{PatKind, Pattern};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(in crate::compiler::syntax::parse) fn pattern_one(&mut self) -> PResult<Pattern> {
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
            _ if self.at_reserved()
                && matches!(self.peek(1), TokenKind::Ident | TokenKind::Kw(Keyword::Mut)) =>
            {
                /* The lexer reported a word before a pattern, as `ref x`: read the pattern. */
                self.bump();
                return self.pattern_one();
            }
            _ if self.at_path_start() || self.at_primitive_path() => self.pattern_path()?,
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
