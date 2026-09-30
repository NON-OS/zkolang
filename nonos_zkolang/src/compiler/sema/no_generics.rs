/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Generic arguments on a path to an item that takes none: every item this build checks. */

use alloc::format;

use super::cx::Sema;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::Path;

impl<'a> Sema<'a> {
    /** Report generic arguments on the path `p` (E0701). */
    pub(crate) fn no_generics(&mut self, p: &Path) {
        if let Some(s) = p.segments.iter().find(|s| s.generics.is_some()) {
            let d = Diagnostic::error(
                Code::WRONG_GENERICS,
                format!("`{}` takes no generic arguments", s.ident.name),
                p.span,
                "generic arguments given",
            );
            self.diags.push(d);
        }
    }
}
