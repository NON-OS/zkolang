/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One step into a place: a tuple field, or an element at an index. */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::super::structs::Key;

use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::Proj;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Take the step `step` into a value of type `ty`. */
    pub(super) fn project(
        &mut self,
        ty: TyId,
        step: &'a Expr,
        mut proj: Vec<Proj>,
    ) -> (TyId, Vec<Proj>) {
        let next = match (&step.kind, self.kind(ty)) {
            (ExprKind::TupleField(_, i, _), _) => {
                self.field_of(ty, Key::Pos(*i), step.span).map(|(i, t)| {
                    proj.push(Proj::TupleField(i));
                    t
                })
            }
            (ExprKind::Field(_, name), _) => self
                .field_of(ty, Key::Name(&name.name), step.span)
                .map(|(i, t)| {
                    proj.push(Proj::TupleField(i));
                    t
                }),
            (ExprKind::Index(_, i), TyKind::Array(el, _)) => {
                proj.push(Proj::Index(self.expr(i, Some(Types::int(IntTy::Usize)))));
                Some(el)
            }
            (_, TyKind::Error) => None,
            _ => {
                let what = match &step.kind {
                    ExprKind::TupleField(_, i, _) => format!("{i}"),
                    _ => alloc::string::String::from("[]"),
                };
                self.no_field(ty, &what, step.span);
                None
            }
        };
        (next.unwrap_or(Types::ERROR), proj)
    }
}
