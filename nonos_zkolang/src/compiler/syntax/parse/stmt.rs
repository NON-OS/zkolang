/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One statement, dispatched on its first token. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Expr, Stmt};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

/** A parsed statement, or an expression that ended at the block's `}` and is its value. */
pub(super) enum StmtOrTail {
    Stmt(Stmt),
    Tail(Expr),
}

impl<'a> Parser<'a> {
    /** One statement, or the expression that may be the block's tail. */
    pub(super) fn stmt(&mut self) -> PResult<Option<StmtOrTail>> {
        self.stmt_attrs()?;
        /* Attributes just before the block's end, reported, stand before nothing. */
        if self.at(TokenKind::RBrace) {
            return Ok(None);
        }
        let start = self.span();
        if self.skip_reserved_stmt() {
            return Ok(None);
        }
        let stmt = match self.kind() {
            TokenKind::Semi => self.stmt_empty(start),
            TokenKind::Kw(Keyword::Let) => self.stmt_let(start)?,
            TokenKind::Kw(Keyword::Assert) => self.stmt_assert(start)?,
            _ => return self.stmt_item_or_expr(start),
        };
        Ok(Some(StmtOrTail::Stmt(stmt)))
    }
}
