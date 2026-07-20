/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Resolve a loop variable to its compile-time value.

use super::state::Compiler;

impl Compiler {
    /// The value of a loop variable if `name` is one, innermost loop first. A loop
    /// variable shadows a same-named binding while its loop is active.
    pub(crate) fn loop_const(&self, name: &str) -> Option<u64> {
        self.loop_consts
            .iter()
            .rev()
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, v)| *v)
    }
}
