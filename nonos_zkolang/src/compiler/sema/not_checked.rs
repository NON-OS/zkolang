/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The forms this build does not check yet (E0904): each is reported, so none passes as
 * checked. The code goes when the stage that checks them lands.
 */

use alloc::format;

use super::cx::Sema;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Item, ItemKind};

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
            ItemKind::Struct(s) => ("generic structs", s.name.span),
            ItemKind::Enum(e) => ("enums", e.name.span),
            ItemKind::TypeAlias(t) => ("generic type aliases", t.name.span),
            ItemKind::Mod(m) => ("modules in their own files", m.name.span),
            ItemKind::Impl(_) => ("generic impl blocks", item.span),
            ItemKind::Const(c) => ("this constant", c.name.span),
            ItemKind::Use(_) => ("this import", item.span),
        };
        self.not_checked(what, at);
    }
}
