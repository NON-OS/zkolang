/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Primary expressions a keyword starts: `return` and `declassify`, and a misplaced `assert`. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * `return`, `return e` or `declassify(e)`. Any other token that reaches here starts no
     * expression and is reported, `assert` with a hint that it is a statement.
     */
    pub(super) fn primary_keyword(&mut self) -> PResult<Expr> {
        let start = self.span();
        match self.kind() {
            TokenKind::Kw(Keyword::Return) => {
                self.bump();
                let value = if self.at_expr_end() {
                    None
                } else {
                    Some(Box::new(self.expr()?))
                };
                let span = start.to(self.prev_span());
                Ok(self.mk(ExprKind::Return(value), span))
            }
            TokenKind::Kw(Keyword::Declassify) => {
                self.bump();
                self.expect(TokenKind::LParen)?;
                let inner = self.restricted(false, |p| p.expr())?;
                self.expect(TokenKind::RParen)?;
                let span = start.to(self.prev_span());
                Ok(self.mk(ExprKind::Declassify(Box::new(inner)), span))
            }
            TokenKind::Kw(Keyword::Assert) => {
                let d = Diagnostic::error(
                    Code::UNEXPECTED_TOKEN,
                    "expected an expression, found `assert`",
                    start,
                    "`assert` is a statement",
                )
                .with_help("write `assert cond;` on its own line");
                self.diags.push(d);
                Err(Reported)
            }
            _ => Err(self.unexpected("an expression")),
        }
    }

    /** Whether the current token ends an expression, so a `return` has no value. */
    fn at_expr_end(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::Semi
                | TokenKind::RBrace
                | TokenKind::RParen
                | TokenKind::RBracket
                | TokenKind::Comma
                | TokenKind::Eof
        )
    }
}
