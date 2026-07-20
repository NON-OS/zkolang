/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Resolve a bound name to its register.

use super::state::Compiler;

impl Compiler {
    /// The register a bound name currently resolves to, newest binding first.
    pub(crate) fn lookup(&self, name: &str) -> Option<u8> {
        self.syms
            .iter()
            .rev()
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, r)| *r)
    }
}
