/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The struct a path names, and the tuple and unit forms of building one. */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::Form;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Path};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `p(args)` for a tuple struct `p`. */
    pub(crate) fn tuple_struct(&mut self, p: &'a Path, args: &'a [Expr], at: Span) -> TExpr {
        let Some(ty) = self.struct_named(p, Form::Tuple) else {
            args.iter().for_each(|a| {
                self.infer(a, None);
            });
            return self.error(at);
        };
        let fields = self.struct_fields(ty);
        self.arity_of(p.last_name(), fields.len(), args.len(), at);
        let mut out = Vec::with_capacity(args.len());
        for (i, a) in args.iter().enumerate() {
            let want = fields.get(i).map(|f| f.1);
            let v = self.expr(a, want);
            out.push((u32::try_from(i).unwrap_or(u32::MAX), v));
        }
        TExpr {
            kind: TExprKind::Record(out),
            ty,
            span: at,
        }
    }

    /** `p` for a unit struct `p`. */
    pub(crate) fn unit_struct(&mut self, p: &'a Path, at: Span) -> TExpr {
        match self.struct_named(p, Form::Unit) {
            Some(ty) => TExpr {
                kind: TExprKind::Record(Vec::new()),
                ty,
                span: at,
            },
            None => self.error(at),
        }
    }
}
