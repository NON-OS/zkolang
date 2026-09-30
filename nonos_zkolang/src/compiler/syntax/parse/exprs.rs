/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Expressions: node construction and assignment. The operators live in `binary` and
 * `unary`, the postfix chain of calls, indexing, fields and method calls in `postfix` and
 * `dot`, and the operands in `primary`.
 */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{AssignOp, BinOp, Expr, ExprKind};
use crate::compiler::syntax::token::TokenKind;

/** The assignment operator a token spells. */
pub(super) fn assign_op(k: TokenKind) -> Option<AssignOp> {
    Some(match k {
        TokenKind::Eq => AssignOp::Assign,
        TokenKind::PlusEq => AssignOp::Compound(BinOp::Add),
        TokenKind::MinusEq => AssignOp::Compound(BinOp::Sub),
        TokenKind::StarEq => AssignOp::Compound(BinOp::Mul),
        TokenKind::SlashEq => AssignOp::Compound(BinOp::Div),
        TokenKind::PercentEq => AssignOp::Compound(BinOp::Rem),
        TokenKind::CaretEq => AssignOp::Compound(BinOp::BitXor),
        TokenKind::AmpEq => AssignOp::Compound(BinOp::BitAnd),
        TokenKind::PipeEq => AssignOp::Compound(BinOp::BitOr),
        TokenKind::ShlEq => AssignOp::Compound(BinOp::Shl),
        TokenKind::ShrEq => AssignOp::Compound(BinOp::Shr),
        _ => return None,
    })
}

impl<'a> Parser<'a> {
    /** Build an expression node. */
    pub(super) fn mk(&mut self, kind: ExprKind, span: Span) -> Expr {
        Expr {
            id: self.id(),
            kind,
            span,
        }
    }

    /** An expression in statement position, where assignment is allowed. */
    pub(super) fn expr_with_assign(&mut self) -> PResult<Expr> {
        let place = self.expr()?;
        let Some(op) = assign_op(self.kind()) else {
            return Ok(place);
        };
        self.bump();
        let value = self.expr()?;
        let span = place.span.to(value.span);
        Ok(self.mk(
            ExprKind::Assign {
                op,
                place: Box::new(place),
                value: Box::new(value),
            },
            span,
        ))
    }
}
