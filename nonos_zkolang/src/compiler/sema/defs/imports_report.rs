/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The report of an import that does not resolve, at the segment at fault. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::{Defs, PathError, PendingImport};
use crate::compiler::diag::{did_you_mean, Code, Diagnostic, Diagnostics};

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
        let d = Diagnostic::error(code, message, at, "unresolved import");
        diags.push(match e {
            PathError::Unresolved(i) => match self.import_near(imp, i) {
                Some(h) => d.with_help(h),
                None => d,
            },
            _ => d,
        });
    }

    /** Help naming what segment `i` of `imp` may have meant. */
    fn import_near(&self, imp: &PendingImport<'a>, i: usize) -> Option<String> {
        let segs: Vec<&str> = imp.segs.iter().map(|s| s.name.as_str()).collect();
        let near = self.names_near(imp.module, imp.root, &segs, i);
        did_you_mean(segs.get(i)?, near)
    }
}
