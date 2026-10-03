/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Unused variables (W0001): a local whose value no expression reads. Assigning to it is
 * not a read; passing it as `&mut` or updating it with `op=` is. A name that starts with
 * `_` is not reported.
 */

use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::tir::LocalId;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Record that the value of local `l` is read. */
    pub(super) fn read_local(&mut self, l: LocalId) {
        if let Some(r) = self.read.get_mut(l.0 as usize) {
            *r = true;
        }
    }

    /** Warn of every local of the body that is never read. */
    pub(crate) fn warn_unused(&mut self) {
        for (l, read) in self.locals.iter().zip(&self.read) {
            if *read || l.name.starts_with('_') {
                continue;
            }
            let what = format!("unused variable `{}`", l.name);
            let d = Diagnostic::warning(Code::UNUSED_VARIABLE, what, l.span, "never read")
                .with_help(format!("name it `_{}` if it is meant to be unused", l.name));
            self.sema.diags.push(d);
        }
    }
}
