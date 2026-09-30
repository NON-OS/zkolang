/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Following the links of joined variables to the one they stand for. */

use super::vars::{IntVars, State};

impl IntVars {
    /**
     * The variable `n` stands for after following its links; every variable on the way is
     * linked to it directly, so a later search is short.
     */
    pub(crate) fn root(&mut self, n: u32) -> u32 {
        let mut r = n;
        for _ in 0..=self.states.len() {
            match self.states.get(r as usize) {
                Some(State::Link(m)) => r = *m,
                _ => break,
            }
        }
        let mut cur = n;
        while cur != r {
            let Some(s) = self.states.get_mut(cur as usize) else {
                break;
            };
            let State::Link(next) = *s else {
                break;
            };
            *s = State::Link(r);
            cur = next;
        }
        r
    }
}
