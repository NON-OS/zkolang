/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Blocks: the statements between `{` and `}`, and the expression that may end them. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Block, Expr, Stmt};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

/** A parsed statement, or an expression that ended at the block's `}` and is its value. */
pub(super) enum StmtOrTail {
    Stmt(Stmt),
    Tail(Expr),
}

impl<'a> Parser<'a> {
    /** `{ stmts tail }`. */
    pub(super) fn block(&mut self) -> PResult<Block> {
        let open = self.expect(TokenKind::LBrace)?;
        let saved = self.no_struct;
        self.no_struct = false;
        let r = self.nested(|p| p.block_body(open.span));
        self.no_struct = saved;
        r
    }

    fn block_body(&mut self, open: Span) -> PResult<Block> {
        let mut stmts = Vec::new();
        let mut tail = None;
        loop {
            match self.kind() {
                TokenKind::RBrace => break,
                TokenKind::Eof => return Err(self.block_unclosed(open)),
                _ => {}
            }
            match self.stmt() {
                Ok(Some(StmtOrTail::Stmt(s))) => stmts.push(s),
                Ok(Some(StmtOrTail::Tail(e))) => tail = Some(e),
                Ok(None) => {}
                Err(_) => self.recover_stmt(),
            }
        }
        let close = self.expect(TokenKind::RBrace)?;
        Ok(Block {
            id: self.id(),
            stmts,
            tail: tail.map(Box::new),
            span: open.to(close.span),
        })
    }

    /** One statement, or the expression that may be the block's tail. */
    fn stmt(&mut self) -> PResult<Option<StmtOrTail>> {
        let start = self.span();
        let stmt = match self.kind() {
            TokenKind::Semi => self.stmt_empty(start),
            TokenKind::Kw(Keyword::Let) => self.stmt_let(start)?,
            TokenKind::Kw(Keyword::Assert) => self.stmt_assert(start)?,
            _ => return self.stmt_item_or_expr(start),
        };
        Ok(Some(StmtOrTail::Stmt(stmt)))
    }
}
