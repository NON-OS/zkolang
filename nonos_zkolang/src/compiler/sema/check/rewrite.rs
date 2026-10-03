/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Rewriting a settled body: each type to what it stands for, and each literal checked. */

use super::cx::FnCx;
use crate::compiler::tir::{TArg, TExpr, TExprKind, TLit};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Rewrite `e` and everything in it. */
    pub(crate) fn rewrite(&mut self, e: &mut TExpr) {
        e.ty = self.zonk(e.ty, true);
        let span = e.span;
        self.fold_negation(e);
        if let TExprKind::Lit(TLit::Int(v)) = e.kind {
            let v = self.check_fits(v, e.ty, e.span);
            e.kind = TExprKind::Lit(TLit::Int(v));
            return;
        }
        match &mut e.kind {
            TExprKind::Unary(_, a)
            | TExprKind::Cast(a)
            | TExprKind::Repeat(a, _)
            | TExprKind::TupleField(a, _)
            | TExprKind::Declassify(a) => self.rewrite(a),
            TExprKind::Chain(first, links) => {
                self.rewrite(first);
                links.iter_mut().for_each(|(_, l)| self.rewrite(l));
            }
            TExprKind::Call(f, args) => {
                args.iter_mut().for_each(|a| match a {
                    TArg::Value(v) => self.rewrite(v),
                    TArg::Place(p) => self.rewrite_place(&mut p.proj, &mut p.ty),
                });
                self.resolve_call(f, span);
            }
            TExprKind::Builtin(_, es) | TExprKind::Tuple(es) | TExprKind::Array(es) => {
                es.iter_mut().for_each(|x| self.rewrite(x))
            }
            TExprKind::Record(fs) | TExprKind::Variant(_, fs) => {
                fs.iter_mut().for_each(|(_, x)| self.rewrite(x))
            }
            TExprKind::Match(s, arms) => self.rewrite_match(s, arms),
            TExprKind::Index(a, i) => {
                self.rewrite(a);
                self.rewrite(i);
            }
            TExprKind::Block(b) => self.rewrite_block(b),
            TExprKind::If(branches, last) => self.rewrite_if(branches, last),
            TExprKind::ForRange { lo, hi, body, .. } => {
                self.rewrite(lo);
                self.rewrite(hi);
                self.rewrite_block(body);
            }
            TExprKind::ForArray { array, body, .. } => {
                self.rewrite(array);
                self.rewrite_block(body);
            }
            TExprKind::While { cond, body, .. } => {
                self.rewrite(cond);
                self.rewrite_block(body);
            }
            TExprKind::Return(Some(v)) => self.rewrite(v),
            TExprKind::Assign { place, value, .. } => {
                self.rewrite_place(&mut place.proj, &mut place.ty);
                self.rewrite(value);
            }
            _ => {}
        }
    }
}
