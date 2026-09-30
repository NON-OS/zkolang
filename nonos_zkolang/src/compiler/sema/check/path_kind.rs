/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A path that names an item of the wrong kind for a value. */

use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefKind;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Path;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report that `p`, naming an item of kind `k`, stands where a value is expected. */
    pub(super) fn not_a_value(&mut self, p: &Path, k: DefKind, at: Span) {
        let name = p.last_name();
        let what = format!("`{name}` is a {}, not a value", k.describe());
        let d = Diagnostic::error(Code::WRONG_KIND, what, at, "not a value");
        let d = match k {
            DefKind::Fn => d.with_help(format!("call it: `{name}(..)`")),
            _ => d,
        };
        self.sema.diags.push(d);
    }
}
