/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Whether a register is still held by a live binding.

use super::state::Compiler;

impl Compiler {
    /// Whether any live binding still holds this register, so it must not be freed.
    /// Both scalar names and array elements count.
    pub(crate) fn reg_in_use(&self, reg: u8) -> bool {
        self.syms.iter().any(|(_, r)| *r == reg)
            || self.arrays.iter().any(|(_, regs)| regs.contains(&reg))
    }
}
