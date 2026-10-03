/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `match` (sections 8.5 and 13): which arm runs depends on the parts of the scrutinee its
 * patterns and those before it test, and on their guards, so what an arm does, and the
 * value of the `match`, is raised by their labels, as for `if`. A binding tests nothing.
 */

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::TyId;
use crate::compiler::tir::{TArm, TExpr};

impl<'p> Flow<'p> {
    /** The labels of the value of `match scrut { arms }`, of type `ty`. */
    pub(super) fn match_(&mut self, scrut: &TExpr, arms: &[TArm], ty: TyId) -> Shape {
        let s = self.expr(scrut);
        let (pc0, mut before) = (self.pc, self.env.clone());
        let (ret0, exit0) = (self.ret_guard, self.exits.last().copied());
        let (mut guard, mut inner) = (Taint::PUBLIC, Taint::PUBLIC);
        let mut merged: Option<alloc::vec::Vec<Shape>> = None;
        let mut value: Option<Shape> = None;
        for arm in arms {
            self.env = before.clone();
            guard = guard.join(self.tested(&arm.pat, &s, scrut.ty));
            self.pc = pc0.join(guard);
            self.bind(&arm.pat, s.clone(), scrut.ty, arm.span);
            if let Some(g) = &arm.guard {
                guard = guard.join(self.expr(g).all());
                before = self.env.clone();
            }
            self.pc = pc0.join(guard);
            let v = self.expr(&arm.body);
            inner = inner.join(self.pc);
            value = Some(value.map_or(v.clone(), |x| x.join(&v)));
            let env = core::mem::take(&mut self.env);
            merged = Some(match merged {
                Some(m) => m.iter().zip(&env).map(|(a, b)| a.join(b)).collect(),
                None => env,
            });
        }
        self.env = merged.unwrap_or(before);
        let left = self.ret_guard != ret0 || self.exits.last().copied() != exit0;
        self.pc = if left { pc0.join(inner) } else { pc0 };
        let none = || Shape::of(ty, Taint::PUBLIC, &self.program.types);
        value.unwrap_or_else(none).raised(guard)
    }
}
