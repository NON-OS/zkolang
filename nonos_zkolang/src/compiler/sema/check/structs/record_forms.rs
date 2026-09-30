/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The tuple and unit forms of building a struct or a variant. */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::Form;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Path};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `p(args)` for a tuple struct or variant `p`. */
    pub(crate) fn tuple_struct(&mut self, p: &'a Path, args: &'a [Expr], at: Span) -> TExpr {
        let Some(s) = self.shape_named(p, Form::Tuple) else {
            args.iter().for_each(|a| {
                self.infer(a, None);
            });
            return self.error(at);
        };
        let fields = self.shape_fields(s);
        let name = self.shape_name(s);
        self.arity_of(&name, fields.len(), args.len(), at);
        let mut out = Vec::with_capacity(args.len());
        for (i, a) in args.iter().enumerate() {
            let want = fields.get(i).map(|f| f.1);
            let v = self.expr(a, want);
            out.push((u32::try_from(i).unwrap_or(u32::MAX), v));
        }
        self.shape_value(s, out, at)
    }

    /** `p` for a unit struct or variant `p`. */
    pub(crate) fn unit_struct(&mut self, p: &'a Path, at: Span) -> TExpr {
        match self.shape_named(p, Form::Unit) {
            Some(s) => self.shape_value(s, Vec::new(), at),
            None => self.error(at),
        }
    }
}
