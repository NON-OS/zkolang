/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The value of a struct or variant, and how messages name one. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::shape::Shape;
use crate::compiler::source::Span;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** How a message names the shape `s`: `S`, or `E::V`. */
    pub(crate) fn shape_name(&mut self, (ty, tag): Shape) -> String {
        let shown = self.show(ty);
        let adt = self.sema.types.adt(ty).filter(|a| a.is_enum);
        match adt.and_then(|a| a.variants.get(tag as usize)) {
            Some(v) => format!("{shown}::{}", v.name),
            None => shown,
        }
    }

    /** The value of the shape `s` built from `fields`, each with its index. */
    pub(super) fn shape_value(&self, s: Shape, fields: Vec<(u32, TExpr)>, at: Span) -> TExpr {
        let kind = match self.sema.types.adt(s.0).is_some_and(|a| a.is_enum) {
            true => TExprKind::Variant(s.1, fields),
            false => TExprKind::Record(fields),
        };
        TExpr {
            kind,
            ty: s.0,
            span: at,
        }
    }
}
