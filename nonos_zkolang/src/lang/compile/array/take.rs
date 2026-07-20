/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Remove and return an array binding.

use alloc::vec::Vec;

use super::super::compiler::Compiler;

impl Compiler {
    /// Remove and return the newest array binding of a name, if any, so a stale entry
    /// never lingers to confuse resolution or the alias check when a name is rebound.
    pub(crate) fn take_array(&mut self, name: &str) -> Option<Vec<u8>> {
        self.arrays
            .iter()
            .rposition(|(n, _)| n.as_str() == name)
            .map(|pos| self.arrays.remove(pos).1)
    }
}
