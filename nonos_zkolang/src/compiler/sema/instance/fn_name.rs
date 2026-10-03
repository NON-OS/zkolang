/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! How messages and tests name a function: its owner, its name, an instance's arguments. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::Sema;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::tir::FnId;

/** How long an instance's arguments may be, written in a message, before they are elided. */
const LONG: usize = 48;

impl<'a> Sema<'a> {
    /** The name of function `f`: `f`, `T::f`, or `f::<u8>` for an instance. */
    pub(crate) fn fn_name(&self, f: FnId) -> String {
        let Some(info) = self.fns.get(f.0 as usize) else {
            return String::new();
        };
        let mut name = match info.owner {
            Some(t) => format!("{}::{}", self.types.display(t), info.decl.name.name),
            None => info.decl.name.name.clone(),
        };
        let own = info.args.get(info.impl_generics.len()..).unwrap_or(&[]);
        if !own.is_empty() {
            let args: Vec<String> = own.iter().map(|g| self.types.arg(*g)).collect();
            let args = args.join(", ");
            name = match args.len() > LONG {
                true => format!("{name}::<..>"),
                false => format!("{name}::<{args}>"),
            };
        }
        name
    }

    /**
     * Report the call at `at` in `from`, whose instance `f` did not check (E0702), unless
     * `from` is an instance of the same function: its own failure is reported already.
     */
    pub(crate) fn instance_failed(&mut self, f: FnId, (from, at): (Option<FnId>, Span)) {
        let def = |g: FnId| self.fns.get(g.0 as usize).map(|i| i.def);
        if from.is_some_and(|g| def(g) == def(f)) {
            return;
        }
        let name = self.fn_name(f);
        let what = format!("`{name}` does not check for these generic arguments");
        let d = Diagnostic::error(Code::INSTANTIATION_FAILED, what, at, "instantiated here")
            .with_note("the errors above point into the generic function's body");
        self.diags.push(d);
    }
}
