/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A depth-first walk to one instruction, placing its dependences first. */

use alloc::vec;
use alloc::vec::Vec;

use super::order::Walk;

impl Walk<'_> {
    /** Place `t` after everything it depends on, depth first, without recursion. */
    pub(super) fn visit(&mut self, t: usize) {
        let mut stack = vec![(t, false)];
        while let Some((u, expanded)) = stack.pop() {
            if self.placed[u] {
                continue;
            }
            if expanded {
                self.place(u);
                continue;
            }
            stack.push((u, true));
            let deps = self.g.deps[u].iter().copied();
            let mut next: Vec<usize> = deps.filter(|&d| !self.placed[d]).collect();
            /* Pushed last is walked first: the operand needing the most registers. */
            next.sort_by_key(|&d| self.g.need[d]);
            stack.extend(next.into_iter().map(|d| (d, false)));
        }
    }
}
