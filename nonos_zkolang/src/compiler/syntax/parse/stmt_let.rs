/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The statements a `;` or a keyword starts: the empty statement, `let` and `assert`. */

use super::parser::{PResult, Parser};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Stmt, StmtKind};
use crate::compiler::syntax::lex::str_literal;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `;` on its own, the `;` current and `start` its span. */
    pub(super) fn stmt_empty(&mut self, start: Span) -> Stmt {
        self.bump();
        Stmt {
            id: self.id(),
            kind: StmtKind::Empty,
            span: start,
        }
    }

    /** `let pat: T = init;`, the `let` current and starting at `start`. */
    pub(super) fn stmt_let(&mut self, start: Span) -> PResult<Stmt> {
        self.bump();
        let pat = self.pattern_no_alt()?;
        let ty = if self.eat(TokenKind::Colon) {
            Some(self.ty()?)
        } else {
            None
        };
        self.expect(TokenKind::Eq)?;
        let init = self.expr()?;
        self.expect(TokenKind::Semi)?;
        let span = start.to(self.prev_span());
        Ok(Stmt {
            id: self.id(),
            kind: StmtKind::Let { pat, ty, init },
            span,
        })
    }

    /** `assert cond;` or `assert cond, "message";`, the `assert` current. */
    pub(super) fn stmt_assert(&mut self, start: Span) -> PResult<Stmt> {
        self.bump();
        let cond = self.expr()?;
        let message = if self.eat(TokenKind::Comma) {
            let t = self.expect(TokenKind::Str)?;
            Some(str_literal(self.text_of(t)))
        } else {
            None
        };
        self.expect(TokenKind::Semi)?;
        let span = start.to(self.prev_span());
        Ok(Stmt {
            id: self.id(),
            kind: StmtKind::Assert { cond, message },
            span,
        })
    }
}
