/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Visiting every expression a block or an expression holds, outer before inner. */

use super::{Block, Expr, ExprKind, ForIter, StmtKind};

impl Block {
    /** Call `f` on every expression of the block's statements and tail. */
    pub fn each_expr(&self, f: &mut dyn FnMut(&Expr)) {
        for s in &self.stmts {
            match &s.kind {
                StmtKind::Let { init: e, .. }
                | StmtKind::Assert { cond: e, .. }
                | StmtKind::Expr { expr: e, .. } => e.each_expr(f),
                StmtKind::Empty => {}
            }
        }
        if let Some(t) = &self.tail {
            t.each_expr(f);
        }
    }
}

impl Expr {
    /** Call `f` on this expression, then on every expression in it. */
    pub fn each_expr(&self, f: &mut dyn FnMut(&Expr)) {
        f(self);
        match &self.kind {
            ExprKind::Struct { fields, .. } => fields
                .iter()
                .flat_map(|x| &x.value)
                .for_each(|x| x.each_expr(f)),
            ExprKind::Tuple(es) | ExprKind::Array(es) => es.iter().for_each(|x| x.each_expr(f)),
            ExprKind::Repeat(e, _)
            | ExprKind::Paren(e)
            | ExprKind::Declassify(e)
            | ExprKind::Unary(_, e)
            | ExprKind::Cast(e, _)
            | ExprKind::Field(e, _)
            | ExprKind::TupleField(e, _, _)
            | ExprKind::RefMut(e) => e.each_expr(f),
            ExprKind::Return(e) => e.iter().for_each(|x| x.each_expr(f)),
            ExprKind::Block(b) => b.each_expr(f),
            ExprKind::If(branches, last) => {
                for br in branches {
                    br.cond.each_expr(f);
                    br.block.each_expr(f);
                }
                last.iter().for_each(|b| b.each_expr(f));
            }
            ExprKind::Match { scrutinee, arms } => {
                scrutinee.each_expr(f);
                for a in arms {
                    a.guard.iter().for_each(|g| g.each_expr(f));
                    a.body.each_expr(f);
                }
            }
            ExprKind::For { iter, body, .. } => {
                match iter {
                    ForIter::Range { lo, hi, .. } => [lo, hi].iter().for_each(|x| x.each_expr(f)),
                    ForIter::Array(e) | ForIter::Enumerate(e) => e.each_expr(f),
                }
                body.each_expr(f);
            }
            ExprKind::While { cond, body, .. } => {
                cond.each_expr(f);
                body.each_expr(f);
            }
            _ => self.each_operand(f),
        }
    }
}
