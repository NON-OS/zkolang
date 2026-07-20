/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Look up a constant table by name.

use super::super::compiler::Compiler;

impl Compiler {
    /// The values of a constant table by name, in declaration order. A scalar
    /// constant is not a table, so it is not returned here.
    pub(crate) fn const_table(&self, name: &str) -> Option<&[u64]> {
        self.consts
            .iter()
            .find(|c| c.name.as_str() == name && !c.scalar)
            .map(|c| c.values.as_slice())
    }
}
