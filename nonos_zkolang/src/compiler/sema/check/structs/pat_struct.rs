/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Struct patterns (section 9.1): `S { f: p, g, .. }`, `S(p, q)` and `S`, each naming the
 * struct of the value it takes. Each field is taken by its pattern, by a binding of its
 * own name, or, after `..`, ignored.
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{Form, TyId};
use crate::compiler::syntax::ast::{Ident, PatKind, Pattern};

/** What takes one field of a struct pattern. */
#[derive(Clone, Copy)]
pub(super) enum Part<'a> {
    Pat(&'a Pattern),
    Name(&'a Ident),
    Ignored,
}

impl<'s, 'a> FnCx<'s, 'a> {
    /** What takes each field of the struct pattern `p` for a value of type `ty`, in order. */
    pub(super) fn struct_parts(&mut self, p: &'a Pattern, ty: TyId) -> Option<Vec<Part<'a>>> {
        let (path, form) = match &p.kind {
            PatKind::Struct { path, .. } => (path, Form::Named),
            PatKind::TupleStruct(path, _) => (path, Form::Tuple),
            PatKind::Path(path) => (path, Form::Unit),
            _ => return None,
        };
        let named = self.struct_named(path, form)?;
        if named != ty {
            let d = Diagnostic::error(
                Code::PATTERN_MISMATCH,
                format!(
                    "a pattern of `{}` for a value of type `{}`",
                    self.show(named),
                    self.show(ty)
                ),
                p.span,
                "does not fit the value",
            );
            self.sema.diags.push(d);
            return None;
        }
        let n = self.struct_fields(ty).len();
        match &p.kind {
            PatKind::TupleStruct(_, ps) if ps.len() == n => {
                Some(ps.iter().map(Part::Pat).collect())
            }
            PatKind::TupleStruct(_, ps) => {
                self.pattern_mismatch(p, ps.len(), ty);
                None
            }
            PatKind::Struct { fields, rest, .. } => Some(self.named_parts(p, fields, *rest, ty)),
            _ => Some(Vec::new()),
        }
    }
}
