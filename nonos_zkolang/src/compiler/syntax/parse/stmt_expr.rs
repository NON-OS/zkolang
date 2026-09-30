/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Expression statements, and the expression that ends a block as its value. */

use super::parser::{starts_item, PResult, Parser};
use super::stmts::StmtOrTail;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

/** Whether an expression is block-shaped, so as a statement it needs no `;`. */
pub(super) fn block_like(e: &Expr) -> bool {
    matches!(
        e.kind,
        ExprKind::Block(_)
            | ExprKind::If { .. }
            | ExprKind::Match { .. }
            | ExprKind::For { .. }
            | ExprKind::While { .. }
    )
}

impl<'a> Parser<'a> {
    /** A statement at `start` that begins with none of `;`, `let` and `assert`. */
    pub(super) fn stmt_item_or_expr(&mut self, start: Span) -> PResult<Option<StmtOrTail>> {
        let k = self.kind();
        if starts_item(k)
            && !(k == TokenKind::Kw(Keyword::Const) && self.peek(1) != TokenKind::Ident)
        {
            self.stmt_item_in_block(start);
            return Ok(None);
        }
        /*
         * A block-shaped expression at the start of a statement stands alone: `if c {} - 1`
         * is not a subtraction.
         */
        let e = if matches!(
            self.kind(),
            TokenKind::LBrace
                | TokenKind::Kw(Keyword::If)
                | TokenKind::Kw(Keyword::Match)
                | TokenKind::Kw(Keyword::For)
                | TokenKind::Kw(Keyword::While)
        ) {
            let e = self.nested(|p| p.primary())?;
            self.after_block_like(e)?
        } else {
            self.expr()?
        };
        let semi = self.eat(TokenKind::Semi);
        if !semi && self.at(TokenKind::RBrace) {
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
