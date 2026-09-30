/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Binary operators by precedence climbing. The operators of one precedence in a row form
 * one node, left-associative, so a long sum costs one nesting level however many terms it
 * has; each such node spends one level while it is open.
 */

use alloc::vec::Vec;

use super::binop::binop;
use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{BinOp, Expr};

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
        let mut chain: Option<(u8, Vec<(BinOp, Expr)>)> = None;
        loop {
            let Some(op) = binop(self.kind()) else {
                break;
            };
            let prec = op.precedence();
            if prec < min {
                break;
            }
            if op.is_comparison() && chain.as_ref().is_some_and(|c| c.0 == prec) {
                return Err(self.chained_comparison());
            }
            self.bump();
            let extends = chain.as_ref().is_some_and(|c| c.0 == prec);
            if !extends {
                /* A new node wraps what came before: one level deeper, spent before the operand. */
                self.enter()?;
            }
            let rhs = self.binary(prec + 1)?;
            match &mut chain {
                Some((_, rest)) if extends => rest.push((op, rhs)),
                _ => {
                    if let Some((_, rest)) = chain.take() {
                        lhs = self.binary_node(lhs, rest);
                    }
                    chain = Some((prec, alloc::vec![(op, rhs)]));
                }
            }
        }
        Ok(match chain {
            Some((_, rest)) => self.binary_node(lhs, rest),
            None => lhs,
        })
    }
}
