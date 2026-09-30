/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Binary operators by precedence climbing. Each link of an operator chain spends one
 * nesting level while the chain is open, because each link deepens the tree even though
 * the parser loops rather than recurses there.
 */

use alloc::boxed::Box;

use super::binop::binop;
use super::parser::{PResult, Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{Expr, ExprKind};

impl<'a> Parser<'a> {
    /** An expression, without assignment. */
    pub(super) fn expr(&mut self) -> PResult<Expr> {
        self.nested(|p| p.binary(0))
    }

    /** Operators binding at least as tightly as `min`. */
    fn binary(&mut self, min: u8) -> PResult<Expr> {
        let base = self.depth;
        let r = self.binary_links(min);
        self.depth = base;
        r
    }

    fn binary_links(&mut self, min: u8) -> PResult<Expr> {
        let mut lhs = self.cast()?;
        let mut last_cmp = false;
        loop {
            let Some(op) = binop(self.kind()) else {
                break;
            };
            let prec = op.precedence();
            if prec < min {
                break;
            }
            if op.is_comparison() && last_cmp {
                let at = self.span();
                self.diags.push(
                    Diagnostic::error(
                        Code::CHAINED_COMPARISON,
                        "comparison operators cannot be chained",
                        at,
                        "a second comparison here",
                    )
                    .with_help("write each comparison separately and join them with `&&`"),
                );
                return Err(Reported);
            }
            self.bump();
            self.enter()?;
            let rhs = self.binary(prec + 1)?;
            let span = lhs.span.to(rhs.span);
            lhs = self.mk(ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)), span);
            last_cmp = op.is_comparison();
        }
        Ok(lhs)
    }
}
