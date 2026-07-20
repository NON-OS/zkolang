/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Array bindings: fixed-size vectors of field values, resolved at compile time. An
//! array names a run of registers, one per element, and a read with a static index
//! selects one of them. Because both the array and the index are known while the
//! program is lowered, an array costs nothing at proof time beyond the registers its
//! live elements hold, and an indexed read is just a reference to one of them. A
//! reassignment reclaims the shadowed array's registers that no live binding holds,
//! so a loop that rebuilds a vector each iteration fits the register file.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::parse::Expr;
use super::super::CompileError;
use super::compiler::Compiler;

impl Compiler {
    /// The element registers of an array binding, newest binding first.
    pub(super) fn lookup_array(&self, name: &str) -> Option<&[u8]> {
        self.arrays
            .iter()
            .rev()
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, r)| r.as_slice())
    }

    /// Bind a name to an array of element registers, reclaiming the registers of any
    /// array it shadows that no live binding still holds. The new elements are pushed
    /// before the reclaim so a register the array keeps is never freed under it.
    pub(super) fn bind_array(&mut self, name: &str, regs: Vec<u8>) {
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

    /// Remove and return the newest array binding of a name, if any. Used when a name
    /// is rebound so a stale entry never lingers to confuse resolution or the alias
    /// check.
    pub(super) fn take_array(&mut self, name: &str) -> Option<Vec<u8>> {
        self.arrays
            .iter()
            .rposition(|(n, _)| n.as_str() == name)
            .map(|pos| self.arrays.remove(pos).1)
    }

    /// Resolve `name[index]` to the register holding that element: the index must
    /// fold to a constant in range. The register is owned by the array, so a read is
    /// not a temporary.
    pub(super) fn array_element(&self, name: &str, index: &Expr) -> Result<u8, CompileError> {
        let regs = self.lookup_array(name).ok_or(CompileError::NotIndexable)?;
        let i = self.const_eval(index)?;
        if i < 0 || i as usize >= regs.len() {
            return Err(CompileError::IndexOutOfBounds);
        }
        Ok(regs[i as usize])
    }
}
