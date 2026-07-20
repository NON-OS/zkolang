/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Point a name at a register.

use alloc::string::String;

use super::state::Compiler;

impl Compiler {
    /// Point a name at a register, replacing its newest binding in place so no
    /// shadowed entry lingers to confuse the alias check, or adding it if new.
    pub(crate) fn rebind(&mut self, name: &str, reg: u8) {
        if let Some(entry) = self.syms.iter_mut().rev().find(|(n, _)| n.as_str() == name) {
            entry.1 = reg;
        } else {
            self.syms.push((String::from(name), reg));
        }
    }
}
