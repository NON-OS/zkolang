/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What follows a `.` in a postfix chain: a tuple field, a named field, or a method call. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::lex::int_literal;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** What follows a `.`: a tuple field, a named field, or a method call. */
    pub(super) fn after_dot(&mut self, e: Expr) -> PResult<Expr> {
        if self.at(TokenKind::Int) {
            let t = self.bump();
            let text = self.text_of(t);
            let index = match int_literal(text) {
                Ok(l)
                    if l.suffix.is_none()
                        && u32::try_from(l.value).is_ok()
                        && !text.contains(['x', 'o', 'b']) =>
                {
                    l.value as u32
                }
                _ => return Err(self.unexpected_at(t.span, "a tuple field index such as `0`")),
            };
            let span = e.span.to(t.span);
            return Ok(self.mk(ExprKind::TupleField(Box::new(e), index, t.span), span));
        }
        let method = self.ident()?;
        let generics = if self.at(TokenKind::ColonColon) && self.peek(1) == TokenKind::Lt {
            self.bump();
            Some(self.generic_args()?)
        } else {
            None
        };
        if self.eat(TokenKind::LParen) {
            let args = self.args(TokenKind::RParen)?;
            let span = e.span.to(self.prev_span());
            return Ok(self.mk(
                ExprKind::MethodCall {
                    receiver: Box::new(e),
                    method,
                    generics,
                    args,
                },
                span,
            ));
        }
        if generics.is_some() {
            return Err(self.unexpected("`(`: generic arguments belong to a method call"));
        }
        let span = e.span.to(method.span);
        Ok(self.mk(ExprKind::Field(Box::new(e), method), span))
    }

    /** Report an unexpected token at a given span. */
    fn unexpected_at(&mut self, span: Span, expected: &str) -> Reported {
        self.diags.push(Diagnostic::error(
            Code::UNEXPECTED_TOKEN,
            alloc::format!("expected {expected}"),
            span,
            alloc::format!("expected {expected}"),
        ));
        Reported
    }
}
