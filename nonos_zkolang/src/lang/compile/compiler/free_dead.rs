/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Return the registers of dead top-level bindings to the pool.

use alloc::string::String;

use super::state::Compiler;

impl Compiler {
    /// Drop every scalar binding whose name is not in `live`, the sorted set of names
    /// read by the rest of the program. The name is removed so a mistaken liveness
    /// result surfaces as an unknown-variable error at the next use rather than a stale
    /// read, and the register returns to the pool only when no other binding, scalar or
    /// array element, still holds it. Array bindings are left in place; only the scalar
    /// register file is reclaimed here.
    pub(crate) fn free_dead(&mut self, live: &[String]) {
        let mut i = 0;
        while i < self.syms.len() {
            if live.binary_search(&self.syms[i].0).is_ok() {
                i += 1;
                continue;
            }
            let reg = self.syms.remove(i).1;
            if !self.reg_in_use(reg) {
                self.free.push(reg);
            }
        }
    }
}
