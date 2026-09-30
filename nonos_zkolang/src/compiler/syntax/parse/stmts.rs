/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Blocks: the statements between `{` and `}`, and the expression that may end them. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::parser::{PResult, Parser, Reported};
use super::stmt::StmtOrTail;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Block;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `{ stmts tail }`. */
    pub(super) fn block(&mut self) -> PResult<Block> {
        let open = self.expect(TokenKind::LBrace)?;
        let saved = self.no_struct;
        self.no_struct = false;
        let r = self.block_body(open.span);
        self.no_struct = saved;
        r
    }

    pub(super) fn block_body(&mut self, open: Span) -> PResult<Block> {
        let mut stmts = Vec::new();
        let mut tail = None;
        loop {
            match self.kind() {
                TokenKind::RBrace => break,
                TokenKind::Eof => {
                    self.report_unclosed(open, "block");
                    return Err(Reported);
                }
                TokenKind::RParen | TokenKind::RBracket => {
                    self.skip_stray_run();
                    continue;
                }
                _ if self.at_outer_item() => {
                    self.report_unclosed(open, "block");
                    let span = open.to(self.prev_span());
                    let (id, tail) = (self.id(), tail.map(Box::new));
                    return Ok(Block {
                        id,
                        stmts,
                        tail,
                        span,
                    });
                }
                _ => {}
            }
            let from = self.pos;
            match self.stmt() {
                Ok(Some(StmtOrTail::Stmt(s))) => stmts.push(s),
                Ok(Some(StmtOrTail::Tail(e))) => tail = Some(e),
                Ok(None) => {}
                Err(_) => self.recover_stmt(from),
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
}
