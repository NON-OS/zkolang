/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Expression statements, and the expression that ends a block as its value. */

use super::block_like::block_like;
use super::parser::{starts_item, PResult, Parser};
use super::stmt::StmtOrTail;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Stmt, StmtKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A statement at `start` that begins with none of `;`, `let` and `assert`. */
    pub(super) fn stmt_item_or_expr(&mut self, start: Span) -> PResult<Option<StmtOrTail>> {
        let k = self.kind();
        let const_item = matches!(self.peek(1), TokenKind::Ident | TokenKind::Kw(Keyword::Fn));
        if starts_item(k) && (k != TokenKind::Kw(Keyword::Const) || const_item) {
            self.stmt_item_in_block(start);
            return Ok(None);
        }
        /*
         * A block-shaped expression at the start of a statement stands alone: `if c {} - 1`
         * is not a subtraction.
         */
        let e = if self.at_block_like() {
            let e = self.nested(|p| p.primary())?;
            self.after_block_like(e)?
        } else {
            self.expr()?
        };
        let semi = self.eat(TokenKind::Semi);
        if !semi && matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof) {
            return Ok(Some(StmtOrTail::Tail(e)));
        }
        if !semi && !block_like(&e) {
            return Err(self.unexpected("`;`"));
        }
        let span = if semi {
            e.span.to(self.prev_span())
        } else {
            e.span
        };
        Ok(Some(StmtOrTail::Stmt(Stmt {
            id: self.id(),
            kind: StmtKind::Expr { expr: e, semi },
            span,
        })))
    }
}
