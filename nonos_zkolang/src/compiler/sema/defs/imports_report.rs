/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The report of an import that does not resolve, at the segment at fault. */

use alloc::format;
use alloc::string::String;

use super::{Defs, PathError, PendingImport};
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};

impl<'a> Defs<'a> {
    /** Report why `imp` does not resolve. */
    pub(super) fn report_import(
        &self,
        imp: &PendingImport<'a>,
        e: PathError,
        diags: &mut Diagnostics,
    ) {
        let seg = |i: usize| imp.segs.get(i).copied();
        let (code, message, at) = match e {
            PathError::Unresolved(i) => (
                Code::UNRESOLVED_NAME,
                seg(i).map(|s| format!("no item named `{}` here", s.name)),
                seg(i),
            ),
            PathError::NotModule(i) => (
                Code::WRONG_KIND,
                seg(i.saturating_sub(1)).map(|s| format!("`{}` is not a module", s.name)),
                seg(i.saturating_sub(1)),
            ),
            PathError::Private(i) => (
                Code::PRIVATE_ITEM,
                seg(i).map(|s| format!("`{}` is private to its module", s.name)),
                seg(i),
            ),
            PathError::Ambiguous(i) => (
                Code::UNRESOLVED_NAME,
                seg(i)
                    .map(|s| format!("`{}` is imported by two globs from different items", s.name)),
                seg(i),
            ),
            PathError::BadRoot => (
                Code::UNRESOLVED_NAME,
                Some(String::from("this `use` names no item")),
                None,
            ),
        };
        let at = at.map_or(imp.span, |s| s.span);
        let message = message.unwrap_or_else(|| String::from("unresolved import"));
        diags.push(Diagnostic::error(code, message, at, "unresolved import"));
    }
}
