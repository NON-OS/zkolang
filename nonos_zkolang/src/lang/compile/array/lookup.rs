/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Look up an array binding by name.

use super::super::compiler::Compiler;

impl Compiler {
    /// The element registers of an array binding, newest binding first.
    pub(crate) fn lookup_array(&self, name: &str) -> Option<&[u8]> {
        self.arrays
            .iter()
            .rev()
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, r)| r.as_slice())
    }
}
