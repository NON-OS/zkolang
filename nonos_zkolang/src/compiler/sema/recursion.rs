/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Recursion (section 10.6). Every call is expanded where it stands, and no function this
 * build checks has a constant generic parameter to end a recursion, so a function that
 * calls itself, directly or through others, is an error (E0700).
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::cx::Sema;
use super::scc::cycles;
use crate::compiler::diag::{Code, Diagnostic};

impl<'a> Sema<'a> {
    /** Report each set of functions that call one another in a cycle. */
    pub(crate) fn check_recursion(&mut self) {
        let graph = self.call_graph();
        for part in cycles(&graph) {
            let Some(&first) = part.first() else {
                continue;
            };
            let names: Vec<String> = part
                .iter()
                .filter_map(|&f| self.fns.get(f).map(|i| format!("`{}`", i.decl.name.name)))
                .collect();
            let at = graph
                .get(first)
                .and_then(|es| es.iter().find(|(g, _)| part.contains(&(g.0 as usize))))
                .map(|(_, s)| *s);
            let Some(at) = at else {
                continue;
            };
            let message = if names.len() == 1 {
                format!("{} calls itself", names.join(""))
            } else {
                format!("{} call one another in a cycle", names.join(", "))
            };
            let d = Diagnostic::error(Code::RECURSION_UNBOUNDED, message, at, "this call recurses")
                .with_help(
                    "every call is expanded at compile time; write the computation with a loop",
                );
            self.diags.push(d);
        }
    }
}
