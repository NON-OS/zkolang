/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Building structs and variants (section 7.11): `S { f: e, g }` or `E::V { f: e, g }`,
 * every field given exactly once and visible here, `g` short for `g: g`; `S(a, b)` for a
 * tuple struct; `S` for a unit one.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::Form;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{FieldInit, Path};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `p { inits }`, for a struct or a variant with named fields. */
    pub(crate) fn struct_lit(&mut self, p: &'a Path, inits: &'a [FieldInit], at: Span) -> TExpr {
        let Some(s) = self.shape_named(p, Form::Named) else {
            inits.iter().filter_map(|f| f.value.as_ref()).for_each(|v| {
                self.infer(v, None);
            });
            return self.error(at);
        };
        let names: Vec<Option<String>> = self.shape_fields(s).into_iter().map(|f| f.0).collect();
        let mut given = vec![false; names.len()];
        let mut out = Vec::with_capacity(inits.len());
        for init in inits {
            let field = self.shape_field(s, &init.name.name, init.name.span);
            let want = field.map(|f| f.1);
            let value = match &init.value {
                Some(v) => self.expr(v, want),
                None => self.shorthand(&init.name, want),
            };
            let Some((i, _)) = field else {
                continue;
            };
            if given
                .get_mut(i as usize)
                .is_some_and(|g| core::mem::replace(g, true))
            {
                let what = format!("the field `{}` is given twice", init.name.name);
                let d = Diagnostic::error(Code::DUPLICATE_FIELD, what, init.span, "given again");
                self.sema.diags.push(d);
            }
            out.push((i, value));
        }
        let missing: Vec<String> = names
            .iter()
            .zip(&given)
            .filter(|(_, g)| !**g)
            .filter_map(|(n, _)| n.as_ref().map(|n| format!("`{n}`")))
            .collect();
        if !missing.is_empty() {
            let shown = self.shape_name(s);
            let what = format!("the literal of `{shown}` leaves out {}", missing.join(", "));
            let d = Diagnostic::error(Code::MISSING_FIELD, what, at, "every field is given once");
            self.sema.diags.push(d);
        }
        self.shape_value(s, out, at)
    }
}
