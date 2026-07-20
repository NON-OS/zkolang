/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Return a value's register to the free pool.

use super::state::Compiler;
use super::val::Val;

impl Compiler {
    /// Return a value's register to the pool if it was a temporary.
    pub(crate) fn release(&mut self, v: &Val) {
        if v.temp {
            self.free.push(v.reg);
        }
    }
}
