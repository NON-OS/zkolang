/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The forms this build does not check yet (E0904): each is reported, so none passes as
 * checked. The code goes when the stage that checks them lands. A module whose file was
 * never loaded is reported too (E0203).
 */

use alloc::format;

use super::cx::Sema;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Ident, Item, ItemKind};

impl<'a> Sema<'a> {
    /** Report `what`, a form this build does not check, at `at` (E0904). */
    pub(crate) fn not_checked(&mut self, what: &str, at: Span) {
        let d = Diagnostic::error(
            Code::UNSUPPORTED,
            format!("this build does not check {what} yet"),
            at,
            "not checked yet",
        );
        self.diags.push(d);
    }

    /** Report an item this build does not check. */
    pub(crate) fn not_yet(&mut self, item: &Item) {
        let (what, at): (&str, Span) = match &item.kind {
            ItemKind::Fn(f) => ("generic functions", f.name.span),
            ItemKind::Mod(m) => ("this module", m.name.span),
            ItemKind::Impl(_) => ("generic impl blocks", item.span),
            ItemKind::Const(c) => ("this constant", c.name.span),
            ItemKind::Use(_) => ("this import", item.span),
            ItemKind::Struct(_) | ItemKind::Enum(_) | ItemKind::TypeAlias(_) => {
                ("this item", item.span)
            }
        };
        self.not_checked(what, at);
    }

    /** Report the module `name`, declared `mod name;` but loaded from no file (E0203). */
    pub(crate) fn unloaded(&mut self, name: &Ident) {
        let what = format!("no file is loaded for module `{}`", name.name);
        let d = Diagnostic::error(Code::MODULE_NOT_FOUND, what, name.span, "declared here")
            .with_help("a crate of several files is loaded from its root file, which reads them");
        self.diags.push(d);
    }
}
