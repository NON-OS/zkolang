/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Hide arrays out of scope and restore them. A call's body sees only the arrays it was
 * passed, and a block local hides an outer array of its name. A hidden array's registers
 * stay held, so nothing in the inner scope can free them under the outer one.
 */

use super::state::Compiler;

impl Compiler {
    /** Hide the newest array bound to `name`, if any. */
    pub(crate) fn hide_array(&mut self, name: &str) {
        if let Some(pos) = self.arrays.iter().rposition(|(n, _)| n.as_str() == name) {
            let entry = self.arrays.remove(pos);
            self.hidden_arrays.push(entry);
        }
    }

    /** Hide every array in scope, for a call's body. */
    pub(crate) fn hide_all_arrays(&mut self) {
        let all = core::mem::take(&mut self.arrays);
        self.hidden_arrays.extend(all);
    }

    /** Return the arrays hidden since `mark` to scope, in the order they were hidden. */
    pub(crate) fn restore_hidden(&mut self, mark: usize) {
        let back = self
            .hidden_arrays
            .split_off(mark.min(self.hidden_arrays.len()));
        self.arrays.extend(back);
    }
}
