/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What a free variable may become: whether it defaults to `usize`, and whether it is general. */

use super::vars::{IntVars, State};

impl IntVars {
    /** Whether the free variable `n` defaults to `usize`. */
    pub(crate) fn usize_default(&mut self, n: u32) -> bool {
        let r = self.root(n);
        let s = self.states.get(r as usize);
        matches!(
            s,
            Some(State::Free {
                usize_default: true,
                ..
            })
        )
    }
}
