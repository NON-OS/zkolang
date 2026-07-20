/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Remove a scalar binding.

use super::state::Compiler;

impl Compiler {
    /// Remove and return the newest scalar binding of a name, if any. Used when a
    /// name becomes an array so its old scalar register can be reclaimed.
    pub(crate) fn take_scalar(&mut self, name: &str) -> Option<u8> {
        self.syms
            .iter()
            .rposition(|(n, _)| n.as_str() == name)
            .map(|pos| self.syms.remove(pos).1)
    }
}
