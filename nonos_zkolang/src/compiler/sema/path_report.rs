/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The report of a path in code that does not resolve, at the segment at fault. */

use alloc::format;
use alloc::string::String;

use super::cx::Sema;
use super::defs::PathError;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Path;

impl<'a> Sema<'a> {
    /** Report why the path `p` does not resolve. */
    pub(crate) fn report_path(&mut self, p: &Path, e: PathError) {
        let seg = |i: usize| {
            p.segments
                .get(i)
                .map(|s| (s.ident.name.as_str(), s.ident.span))
        };
        let (code, message, at): (Code, String, Option<(&str, Span)>) = match e {
            PathError::Unresolved(i) => (
                Code::UNRESOLVED_NAME,
                seg(i).map_or(String::new(), |s| {
                    format!("no item or variable named `{}` here", s.0)
                }),
                seg(i),
            ),
            PathError::NotModule(i) => (
                Code::WRONG_KIND,
                seg(i.saturating_sub(1))
                    .map_or(String::new(), |s| format!("`{}` is not a module", s.0)),
                seg(i.saturating_sub(1)),
            ),
            PathError::Private(i) => (
                Code::PRIVATE_ITEM,
                seg(i).map_or(String::new(), |s| {
                    format!("`{}` is private to its module", s.0)
                }),
                seg(i),
            ),
            PathError::Ambiguous(i) => (
                Code::UNRESOLVED_NAME,
                seg(i).map_or(String::new(), |s| {
                    format!("`{}` is imported by two globs from different items", s.0)
                }),
                seg(i),
            ),
            PathError::BadRoot => (
                Code::UNRESOLVED_NAME,
                String::from("this path starts above the crate or at `Self`"),
                None,
            ),
        };
        let at = at.map_or(p.span, |s| s.1);
        self.diags
            .push(Diagnostic::error(code, message, at, "unresolved"));
    }
}
