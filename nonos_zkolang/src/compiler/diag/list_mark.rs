/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Going back to an earlier point of a list, when a check is made again another way. */

use super::Diagnostics;

impl Diagnostics {
    /** How many diagnostics are recorded, to go back to with `rewind`. */
    pub fn mark(&self) -> usize {
        self.items.len()
    }

    /** Forget the diagnostics recorded since `mark` gave `at`. */
    pub fn rewind(&mut self, at: usize) {
        self.items.truncate(at);
    }
}
