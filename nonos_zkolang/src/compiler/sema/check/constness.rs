/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Constant expressions (section 11): literals, constants, operators, conversions, tuples
 * and arrays over them, `if` over them, and calls of a `const fn` with constant arguments.
 */

use crate::compiler::sema::cx::Sema;
use crate::compiler::source::Span;
use crate::compiler::tir::{TArg, TBlock, TExpr, TExprKind};

impl<'a> Sema<'a> {
    /** The first part of `e` that is not constant, if there is one. */
    pub(crate) fn not_const(&self, e: &TExpr) -> Option<Span> {
        match &e.kind {
            TExprKind::Lit(_) | TExprKind::Const(_) | TExprKind::Error => None,
            TExprKind::Unary(_, a)
            | TExprKind::Cast(a)
            | TExprKind::Repeat(a, _)
            | TExprKind::TupleField(a, _) => self.not_const(a),
            TExprKind::Chain(first, links) => self
                .not_const(first)
                .or_else(|| links.iter().find_map(|(_, l)| self.not_const(l))),
            TExprKind::Builtin(_, es) | TExprKind::Tuple(es) | TExprKind::Array(es) => {
                es.iter().find_map(|x| self.not_const(x))
            }
            TExprKind::Index(a, i) => self.not_const(a).or_else(|| self.not_const(i)),
            TExprKind::Block(b) => self.block_not_const(b),
            TExprKind::If(branches, last) => branches
                .iter()
                .find_map(|(c, b)| self.not_const(c).or_else(|| self.block_not_const(b)))
                .or_else(|| last.as_ref().and_then(|b| self.block_not_const(b))),
            TExprKind::Call(f, args) => {
                let is_const = self.fns.get(f.0 as usize).is_some_and(|i| i.decl.is_const);
                if !is_const {
                    return Some(e.span);
                }
                args.iter().find_map(|a| match a {
                    TArg::Value(v) => self.not_const(v),
                    TArg::Place(p) => Some(p.span),
                })
            }
            _ => Some(e.span),
        }
    }

    /** The first part of `b` that is not constant: a block of a tail alone may be. */
    fn block_not_const(&self, b: &TBlock) -> Option<Span> {
        if !b.stmts.is_empty() {
            return Some(b.span);
        }
        b.tail.as_ref().and_then(|t| self.not_const(t))
    }
}
