/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Block-like expressions, which as a statement need no `;` and as an arm no `,`. */

use super::parser::Parser;
use crate::compiler::syntax::ast::{Expr, ExprKind};
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
    /** Whether the current token begins a block-like expression. */
    pub(super) fn at_block_like(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::LBrace
                | TokenKind::Kw(Keyword::If)
                | TokenKind::Kw(Keyword::Match)
                | TokenKind::Kw(Keyword::For)
                | TokenKind::Kw(Keyword::While)
        )
    }
}
