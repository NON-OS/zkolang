/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The bounds on instances of a generic function (section 10.6): `NESTED` of one template
 * along the calls that make them, and `WRITTEN` parts of arguments written out (E0700).
 */

use alloc::format;

use super::super::cx::Sema;
use super::super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::tir::FnId;

/** How many instances of one generic function may nest along the calls that make them. */
const NESTED: usize = 64;

/** How many parts the generic arguments of one instance may take, written out. */
const WRITTEN: usize = 1024;

impl<'a> Sema<'a> {
    /** Whether an instance of `def`, named `name`, for `args` may be made at `at` in `from`. */
    pub(crate) fn instance_bounded(
        &mut self,
        (def, name): (DefId, &str),
        args: &[GenArg],
        from: Option<FnId>,
        at: Span,
    ) -> bool {
        let mut budget = WRITTEN;
        let small = args.iter().all(|g| match g {
            GenArg::Type(e) => self.types.within(*e, &mut budget),
            GenArg::Const(_) => true,
        });
        let (what, help) = match (small, self.nesting(def, from) >= NESTED) {
            (true, false) => return true,
            (false, _) => (
                format!("the generic arguments of `{name}` grow without end"),
                "a generic function that calls itself gives the same arguments, or stops on a constant condition",
            ),
            (true, true) => (
                format!("the instances of `{name}` nest more than {NESTED} deep"),
                "a recursion ends on a constant condition, such as `if N == 0` on a constant generic parameter",
            ),
        };
        let d = Diagnostic::error(
            Code::RECURSION_UNBOUNDED,
            what,
            at,
            "instantiated again here",
        );
        self.diags.push(d.with_help(help));
        false
    }

    /** How many instances of the item `def` the calls that reach `from` go through. */
    fn nesting(&self, def: DefId, mut from: Option<FnId>) -> usize {
        let (mut n, mut steps) = (0, 0);
        while let Some(info) = from.and_then(|f| self.fns.get(f.0 as usize)) {
            n += usize::from(info.def == def);
            from = info.origin.and_then(|o| o.0);
            steps += 1;
            if steps > self.fns.len() {
                break;
            }
        }
        n
    }
}
