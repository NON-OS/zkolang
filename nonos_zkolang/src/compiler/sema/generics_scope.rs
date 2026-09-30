/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Generic parameters in scope (section 4.4): a path of one name, written where a type or
 * a constant stands, names a generic parameter before any item of that name.
 */

use alloc::format;

use super::cx::Sema;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::GenArg;
use crate::compiler::syntax::ast::Path;

impl<'a> Sema<'a> {
    /** What the generic parameter `p` names stands for, if `p` names one in scope. */
    pub(crate) fn generic_named(&self, p: &Path) -> Option<GenArg> {
        let id = p.as_ident()?;
        let found = self.generics.iter().rev().find(|(n, _)| *n == id.name);
        found.map(|(_, a)| *a)
    }

    /** Report the generic parameter `p` names, of the other kind than `want` names. */
    pub(crate) fn generic_kind(&mut self, p: &Path, want: &str) {
        let what = format!(
            "`{}` is a generic parameter of the other kind, not {want}",
            p.last_name()
        );
        let d = Diagnostic::error(Code::WRONG_KIND, what, p.span, "a generic parameter");
        self.diags.push(d);
    }
}
