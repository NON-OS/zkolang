/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `S { f: p, g, .. }`: each field named once, taken by its pattern or by a binding of its
 * name; the fields not named are ignored after `..` and reported without it (E0315).
 */

use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::pat_struct::Part;
use super::shape::Shape;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{FieldPat, Pattern};

impl<'s, 'a> FnCx<'s, 'a> {
    /** What takes each field of the shape `s` in the named pattern `p`, in order. */
    pub(super) fn named_parts(
        &mut self,
        p: &'a Pattern,
        fields: &'a [FieldPat],
        rest: bool,
        s: Shape,
    ) -> Vec<Part<'a>> {
        let n = self.shape_fields(s).len();
        let mut parts: Vec<Option<Part<'a>>> = vec![None; n];
        for fp in fields {
            let Some((i, _)) = self.shape_field(s, &fp.name.name, fp.name.span) else {
                continue;
            };
            let part = fp.pat.as_ref().map_or(Part::Name(&fp.name), Part::Pat);
            if parts
                .get_mut(i as usize)
                .is_some_and(|x| x.replace(part).is_some())
            {
                let what = format!("the field `{}` is named twice", fp.name.name);
                let d = Diagnostic::error(Code::DUPLICATE_FIELD, what, fp.span, "named again");
                self.sema.diags.push(d);
            }
        }
        if !rest && parts.iter().any(Option::is_none) {
            let what = format!("the pattern of `{}` leaves out fields", self.shape_name(s));
            let d = Diagnostic::error(
                Code::MISSING_FIELD,
                what,
                p.span,
                "not every field is named",
            )
            .with_help("end the pattern with `..` to ignore the rest");
            self.sema.diags.push(d);
        }
        parts
            .into_iter()
            .map(|x| x.unwrap_or(Part::Ignored))
            .collect()
    }
}
