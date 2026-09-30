/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A constant argument written as a path: a constant of type `usize`. */

use alloc::format;
use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::Types;
use crate::compiler::syntax::ast::Path;
use crate::compiler::syntax::IntTy;

impl<'a> Sema<'a> {
    /** The value of the constant `p` names in module `m`, which must be a `usize`. */
    pub(super) fn const_path(&mut self, m: DefId, p: &'a Path) -> Option<i128> {
        let usize_ty = Types::int(IntTy::Usize);
        self.no_generics(p);
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let def = match self.defs.resolve(m, p.root, &names) {
            Ok(d) => d,
            Err(e) => {
                self.report_path(p, e);
                return None;
            }
        };
        let Some(c) = self.const_of.get(&def).copied() else {
            let kind = self.defs.get(def).map_or("item", |d| d.kind.describe());
            self.diags.push(Diagnostic::error(
                Code::WRONG_KIND,
                format!("`{}` is a {kind}, not a constant", p.last_name()),
                p.span,
                "not a constant",
            ));
            return None;
        };
        let ty = self.const_ty(c);
        if ty != usize_ty && ty != Types::ERROR {
            let d = Diagnostic::error(
                Code::MISMATCHED_TYPES,
                "mismatched types",
                p.span,
                format!("expected `usize`, found `{}`", self.types.display(ty)),
            );
            self.diags.push(d);
            return None;
        }
        Some(self.const_value(c)?.int())
    }
}
