/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Struct and variant patterns (section 9.1): `S { f: p, g, .. }`, `S(p, q)` and `S`, each
 * naming the struct, or the enum's variant, of the value it takes. Each field is taken by its pattern, by a binding of its
 * own name, or, after `..`, ignored.
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::shape::Shape;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{Form, TyId};
use crate::compiler::syntax::ast::{Ident, PatKind, Pattern};

/** What takes each field of a struct or variant pattern, in order. */
pub(super) type Parts<'a> = Vec<Part<'a>>;

/** What takes one field of a struct pattern. */
#[derive(Clone, Copy)]
pub(super) enum Part<'a> {
    Pat(&'a Pattern),
    Name(&'a Ident),
    Ignored,
}

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The struct or variant the pattern `p` takes, for a value of type `ty`, and what takes
     * each of its fields, in order; `None` once reported.
     */
    pub(super) fn struct_parts(&mut self, p: &'a Pattern, ty: TyId) -> Option<(Shape, Parts<'a>)> {
        let (path, form) = match &p.kind {
            PatKind::Struct { path, .. } => (path, Form::Named),
            PatKind::TupleStruct(path, _) => (path, Form::Tuple),
            PatKind::Path(path) => (path, Form::Unit),
            _ => return None,
        };
        let s = self.shape_named(path, form)?;
        if !self.unify(s.0, ty) {
            let d = Diagnostic::error(
                Code::PATTERN_MISMATCH,
                format!(
                    "a pattern of `{}` for a value of type `{}`",
                    self.shape_name(s),
                    self.show(ty)
                ),
                p.span,
                "does not fit the value",
            );
            self.sema.diags.push(d);
            return None;
        }
        let n = self.shape_fields(s).len();
        let parts = match &p.kind {
            PatKind::TupleStruct(_, ps) if ps.len() == n => ps.iter().map(Part::Pat).collect(),
            PatKind::TupleStruct(_, ps) => {
                self.pattern_mismatch(p, ps.len(), ty);
                return None;
            }
            PatKind::Struct { fields, rest, .. } => self.named_parts(p, fields, *rest, s),
            _ => Vec::new(),
        };
        Some((s, parts))
    }
}
