/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Look up a scalar constant by name.

use super::super::compiler::Compiler;

impl Compiler {
    /// The value of a scalar constant by name, if one is declared. A table is not a
    /// scalar, so it is not returned here.
    pub(crate) fn scalar_const(&self, name: &str) -> Option<u64> {
        self.consts
            .iter()
            .find(|c| c.name.as_str() == name && c.scalar)
            .and_then(|c| c.values.first().copied())
    }
}
