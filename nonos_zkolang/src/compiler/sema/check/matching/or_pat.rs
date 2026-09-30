/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Alternatives `p | q` (section 9.1): the first binds its names, and every later one binds
 * the same names to the same locals.
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::Pattern;
use crate::compiler::tir::{Labels, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The alternatives `alts`, each taking a value of type `ty`. */
    pub(super) fn or_pat(
        &mut self,
        alts: &'a [Pattern],
        ty: TyId,
        labels: &Labels,
        path: &mut Vec<u32>,
    ) -> TPat {
        let start = self.pats.bound.len();
        let mut first_end = start;
        let mut out = Vec::with_capacity(alts.len());
        for (i, alt) in alts.iter().enumerate() {
            if i == 0 {
                out.push(self.pat(alt, ty, labels, path));
                first_end = self.pats.bound.len();
                continue;
            }
            let first = self.pats.bound.get(start..first_end).unwrap_or_default();
            let names = first.iter().map(|(n, l)| (n.clone(), *l, false)).collect();
            self.pats.reuse.push(names);
            out.push(self.pat(alt, ty, labels, path));
            let names = self.pats.reuse.pop().unwrap_or_default();
            for (name, ..) in names.into_iter().filter(|e| !e.2) {
                self.unbound_in(alt, &name);
            }
            self.pats.bound.truncate(first_end);
        }
        TPat::Or(out)
    }

    /** Report the alternative `alt`, which does not bind `name` (E0403). */
    fn unbound_in(&mut self, alt: &Pattern, name: &str) {
        let what = format!("`{name}` is not bound in this alternative");
        let d = Diagnostic::error(Code::PATTERN_BINDINGS, what, alt.span, "binds fewer names");
        let help = "alternatives `p | q` bind the same names, at the same types and mutability";
        self.sema.diags.push(d.with_help(help));
    }
}
