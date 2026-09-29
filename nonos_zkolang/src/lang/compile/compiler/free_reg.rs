/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Return a register to the free pool. */

use super::state::Compiler;

impl Compiler {
    /**
     * Return `r` to the pool unless it is already there. An array can hold one register in
     * several slots, and a pool holding a register twice hands it to two live values.
     */
    pub(crate) fn free_reg(&mut self, r: u8) {
        if !self.free.contains(&r) {
            self.free.push(r);
        }
    }
}
