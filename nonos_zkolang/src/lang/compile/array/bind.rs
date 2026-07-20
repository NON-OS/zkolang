/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bind a name to an array of element registers.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::compiler::Compiler;

impl Compiler {
    /// Bind a name to an array, reclaiming the registers of any array it shadows that
    /// no live binding still holds. The new elements are pushed before the reclaim, so
    /// a register the array keeps is never freed under it.
    pub(crate) fn bind_array(&mut self, name: &str, regs: Vec<u8>) {
        let old = self.take_array(name);
        self.arrays.push((String::from(name), regs.clone()));
        if let Some(old) = old {
            for r in old {
                if !regs.contains(&r) && !self.reg_in_use(r) {
                    self.free.push(r);
                }
            }
        }
    }
}
