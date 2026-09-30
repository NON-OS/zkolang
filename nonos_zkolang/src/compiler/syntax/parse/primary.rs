/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Primary expressions: the operands that operators and postfix chains apply to. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * A literal, a path, a struct literal, a parenthesized expression or tuple, an array, a
     * block, a control-flow expression, or `declassify`.
     */
    pub(super) fn primary(&mut self) -> PResult<Expr> {
        match self.kind() {
            TokenKind::Int
            | TokenKind::Str
            | TokenKind::Kw(Keyword::True)
            | TokenKind::Kw(Keyword::False)
            | TokenKind::Kw(Keyword::Break)
            | TokenKind::Kw(Keyword::Continue)
            | TokenKind::Error => self.primary_token(),
            TokenKind::Kw(k) if self.peek(1) == TokenKind::ColonColon && primitive(k) => {
                let path = self.primitive_path()?;
                let span = path.span;
                Ok(self.mk(ExprKind::Path(path), span))
            }
            _ if self.at_path_start() => self.path_expr(),
            TokenKind::LParen => self.paren_or_tuple(),
            TokenKind::LBracket => self.array(),
            TokenKind::LBrace => {
                let b = self.block()?;
                let span = b.span;
                Ok(self.mk(ExprKind::Block(Box::new(b)), span))
            }
            TokenKind::Pound => {
                self.expr_attrs()?;
                self.primary()
            }
            TokenKind::Kw(Keyword::If) => self.if_expr(),
            TokenKind::Kw(Keyword::Match) => self.match_expr(),
            TokenKind::Kw(Keyword::For) => self.for_expr(),
            TokenKind::Kw(Keyword::While) => self.while_expr(),
            _ => self.primary_keyword(),
        }
    }
}

/** Whether a keyword names a primitive type that can start an expression path. */
fn primitive(k: Keyword) -> bool {
    matches!(
        k,
        Keyword::Field
            | Keyword::Bool
            | Keyword::U8
            | Keyword::U16
            | Keyword::U32
            | Keyword::U64
            | Keyword::I8
            | Keyword::I16
            | Keyword::I32
            | Keyword::I64
            | Keyword::Usize
    )
}
