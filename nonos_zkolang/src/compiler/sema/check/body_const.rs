/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Constants written in a body: a length, count or `limit`, which must be constant, and
 * the report of a part that must be constant and is not (E0500).
 */

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::ConstArg;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The value of a constant argument written in the body: a local's name there is a
     * variable, not a constant.
     */
    pub(crate) fn const_arg(&mut self, a: &'a ConstArg) -> Option<u32> {
        if let ConstArg::Path(p) = a {
            if p.as_ident().is_some_and(|i| self.lookup(&i.name).is_some()) {
                self.not_constant(
                    p.span,
                    "a length, count or `limit` is a constant expression",
                );
                return None;
            }
        }
        self.sema.const_usize(self.module, a)
    }

    /** Report a part at `at` that must be constant and is not (E0500). */
    pub(crate) fn not_constant(&mut self, at: Span, why: &str) {
        let d = Diagnostic::error(
            Code::NOT_CONSTANT,
            alloc::string::String::from(why),
            at,
            "not a constant expression",
        )
        .with_help(
            "a constant expression uses literals, constants and `const fn` calls, not variables",
        );
        self.sema.diags.push(d);
    }
}
