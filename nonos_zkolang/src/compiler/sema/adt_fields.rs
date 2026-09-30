/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The fields of a struct or variant declaration, their types, labels and visibility; and
 * the pass that lowers every struct declared.
 */

use alloc::format;
use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::{DefId, DefKind};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::AdtField;
use crate::compiler::syntax::ast::{FieldDecl, Visibility};
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /**
     * The fields `decls`, written in module `m`, and the labels they write, each under its
     * field's index. A name declared twice is reported (E0314).
     */
    pub(super) fn fields(&mut self, m: DefId, decls: &'a [FieldDecl]) -> (Vec<AdtField>, Labels) {
        let (mut fields, mut labels) = (Vec::with_capacity(decls.len()), Labels::default());
        for (i, f) in decls.iter().enumerate() {
            let name = f.name.as_ref().map(|n| n.name.clone());
            let named = |g: &FieldDecl| g.name.as_ref().map(|n| &n.name) == name.as_ref();
            if let (true, Some(n)) = (decls[..i].iter().any(named), &f.name) {
                let d = Diagnostic::error(
                    Code::DUPLICATE_FIELD,
                    format!("the field `{}` is declared twice", n.name),
                    n.span,
                    "declared again",
                );
                self.diags.push(d);
            }
            let (ty, own) = self.lower_ty(m, &f.ty);
            let at = u32::try_from(i).unwrap_or(u32::MAX);
            for (q, l) in &own.0 {
                labels.0.push(([&[at][..], q].concat(), *l));
            }
            let public = f.vis == Visibility::Public;
            fields.push(AdtField {
                name,
                ty,
                labels: own,
                public,
            });
        }
        (fields, labels)
    }

    /** Lower every struct of the program, so each declaration is checked. */
    pub(super) fn structs(&mut self) {
        let kinds: Vec<_> = self.defs.defs.iter().map(|d| d.kind).collect();
        for (i, k) in kinds.into_iter().enumerate() {
            if k == DefKind::Struct {
                self.struct_ty(DefId(u32::try_from(i).unwrap_or(u32::MAX)));
            }
        }
    }

    /** Report that the struct `def` contains itself (E0313). */
    pub(super) fn contains_itself(&mut self, def: DefId) {
        if let Some(d) = self.defs.get(def) {
            let message = format!("the struct `{}` contains itself", d.name);
            let d = Diagnostic::error(Code::RECURSIVE_TYPE, message, d.span, "here");
            self.diags
                .push(d.with_help("a field cannot hold its own struct, however deep"));
        }
    }
}
