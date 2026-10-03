/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Charging one row of a cost report to the functions whose code it is. */

use super::cost::Cost;
use crate::compiler::tir::FnId;

impl Cost {
    /**
     * Charge a row to `chain`, the functions inlined to reach its code, outermost first:
     * each has it inclusive, the last exclusive, at `live` registers live. A row of no
     * function is overhead.
     */
    pub(super) fn charge(&mut self, chain: &[FnId], live: usize) {
        let Some((&own, _)) = chain.split_last() else {
            self.overhead = self.overhead.saturating_add(1);
            return;
        };
        for (k, f) in chain.iter().enumerate() {
            /* A function met again further in, by recursion, is counted once. */
            if !chain.iter().take(k).any(|g| g == f) {
                let e = self.fns.entry(*f).or_default();
                e.inclusive = e.inclusive.saturating_add(1);
            }
        }
        let e = self.fns.entry(own).or_default();
        e.exclusive = e.exclusive.saturating_add(1);
        e.peak = e.peak.max(live);
    }
}
