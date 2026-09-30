/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Visiting the expressions directly inside an expression, in evaluation order. */

use super::visit_block::each_index;
use super::{TArg, TExpr, TExprKind};

impl TExpr {
    /** Call `f` on each expression directly inside `self`, left to right. */
    pub fn each_child(&self, f: &mut dyn FnMut(&TExpr)) {
        match &self.kind {
            TExprKind::Unary(_, a)
            | TExprKind::Cast(a)
            | TExprKind::Repeat(a, _)
            | TExprKind::TupleField(a, _)
            | TExprKind::Declassify(a) => f(a),
            TExprKind::Chain(first, links) => {
                f(first);
                links.iter().for_each(|(_, l)| f(l));
            }
            TExprKind::Call(_, args) => args.iter().for_each(|a| match a {
                TArg::Value(v) => f(v),
                TArg::Place(p) => p.proj.iter().for_each(|x| each_index(x, f)),
            }),
            TExprKind::Builtin(_, es) | TExprKind::Tuple(es) | TExprKind::Array(es) => {
                for x in es {
                    f(x);
                }
            }
            TExprKind::Record(fs) | TExprKind::Variant(_, fs) => fs.iter().for_each(|(_, x)| f(x)),
            TExprKind::Match(s, arms) => {
                f(s);
                arms.iter().for_each(|a| a.each_expr(f));
            }
            TExprKind::Index(a, i) => {
                f(a);
                f(i);
            }
            TExprKind::Block(b) => b.each_expr(f),
            TExprKind::If(branches, last) => {
                for (c, b) in branches {
                    f(c);
                    b.each_expr(f);
                }
                if let Some(b) = last {
                    b.each_expr(f);
                }
            }
            TExprKind::ForRange { lo, hi, body, .. } => {
                f(lo);
                f(hi);
                body.each_expr(f);
            }
            TExprKind::ForArray { array, body, .. } => {
                f(array);
                body.each_expr(f);
            }
            TExprKind::While { cond, body, .. } => {
                f(cond);
                body.each_expr(f);
            }
            TExprKind::Return(Some(v)) => f(v),
            TExprKind::Assign { place, value, .. } => {
                place.proj.iter().for_each(|x| each_index(x, f));
                f(value);
            }
            _ => {}
        }
    }
}
