/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The body of an `if`, `for` or `while`, recovered when its `{` is missing. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use super::stmt::StmtOrTail;
use crate::compiler::syntax::ast::Block;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * The block of an `if`, `for` or `while`. When its `{` is missing, the report is made
     * once and the body is read as the source most likely meant it: from a `{` later on
     * the same line, after the stray text before it; else, in a file with a `{` missing,
     * up to the `}` that closes it; else as the one statement that follows.
     */
    pub(super) fn body_block(&mut self) -> PResult<Block> {
        if self.at(TokenKind::LBrace) {
            return self.block();
        }
        let missing = self.unexpected("`{`");
        if self.brace_on_line() {
            while !self.at(TokenKind::LBrace) {
                self.bump();
            }
            return self.block();
        }
        let open = self.span();
        if self.layout.missing_openers {
            return self.block_body(open);
        }
        let (stmts, tail) = match self.stmt() {
            Ok(Some(StmtOrTail::Stmt(s))) => (alloc::vec![s], None),
            Ok(Some(StmtOrTail::Tail(e))) => (alloc::vec![], Some(Box::new(e))),
            _ => return Err(missing),
        };
        let span = open.to(self.prev_span());
        Ok(Block {
            id: self.id(),
            stmts,
            tail,
            span,
        })
    }

    /** Whether a `{` outside brackets follows on the current line, before any `}`. */
    fn brace_on_line(&self) -> bool {
        let mut depth = 0usize;
        let mut prev = self.span();
        for t in self.tokens.get(self.pos..).unwrap_or(&[]) {
            let gap = self
                .text
                .get(prev.hi as usize..t.span.lo as usize)
                .unwrap_or("");
            if gap.contains(['\n', '\r']) {
                return false;
            }
            prev = t.span;
            match t.kind {
                TokenKind::LParen | TokenKind::LBracket => depth += 1,
                TokenKind::RParen | TokenKind::RBracket => depth = depth.saturating_sub(1),
                TokenKind::LBrace if depth == 0 => return true,
                TokenKind::RBrace | TokenKind::Eof => return false,
                _ => {}
            }
        }
        false
    }
}
