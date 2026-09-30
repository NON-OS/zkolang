/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Primary expressions of one token: literals, `break`, `continue`, and lexer errors. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Expr, ExprKind, Lit};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::lex::{int_literal, str_literal};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * The expression one token makes, the token current and one of: an integer, string or
     * boolean literal, `break`, `continue`, or a token the lexer rejected.
     */
    pub(super) fn primary_token(&mut self) -> PResult<Expr> {
        let t = self.bump();
        match t.kind {
            TokenKind::Int => {
                let (value, suffix) = int_literal(self.text_of(t))
                    .map(|l| (l.value, l.suffix))
                    .unwrap_or((0, None));
                Ok(self.mk(
                    ExprKind::Lit(Lit::Int {
                        value,
                        suffix,
                        span: t.span,
                    }),
                    t.span,
                ))
            }
            TokenKind::Str => {
                let value = str_literal(self.text_of(t));
                Ok(self.mk(
                    ExprKind::Lit(Lit::Str {
                        value,
                        span: t.span,
                    }),
                    t.span,
                ))
            }
            TokenKind::Kw(Keyword::True) | TokenKind::Kw(Keyword::False) => {
                let value = t.kind == TokenKind::Kw(Keyword::True);
                Ok(self.mk(
                    ExprKind::Lit(Lit::Bool {
                        value,
                        span: t.span,
                    }),
                    t.span,
                ))
            }
            TokenKind::Kw(Keyword::Break) => Ok(self.mk(ExprKind::Break, t.span)),
            TokenKind::Kw(Keyword::Continue) => Ok(self.mk(ExprKind::Continue, t.span)),
            /* A token the lexer rejected and reported; stand an error node in for it. */
            _ => Ok(self.mk(ExprKind::Error, t.span)),
        }
    }
}
