/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Bind a name to a freshly read input register. */

use super::state::Compiler;

impl Compiler {
    /**
     * Bind `name` to the register an `input` or `secret` just read, the way a `let` rebinds:
     * the name's previous binding, scalar or array, can no longer be named, so its registers
     * go back to the pool when no other live binding holds them. Pushing a second entry
     * instead left the old register held for as long as the name stayed live, which inside
     * a loop leaked one register per iteration.
     */
    pub(crate) fn bind_fresh(&mut self, name: &str, reg: u8) {
        let old = self.lookup(name);
        if let Some(old_array) = self.take_array(name) {
            for r in old_array {
                if r != reg && !self.reg_in_use(r) {
                    self.free.push(r);
                }
            }
        }
        self.rebind(name, reg);
        if let Some(old_reg) = old {
            if old_reg != reg && !self.reg_in_use(old_reg) {
                self.free.push(old_reg);
            }
        }
    }
}
