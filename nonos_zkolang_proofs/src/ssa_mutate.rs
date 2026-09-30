/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Changing an accepted witness's advice, which the VM must then reject: each value by
 * one, and each two neighbours by `+2, -1` and `-2, +1`, which keeps a sum `2 x + y`,
 * the change a decomposition whose bits are not constrained to 0 or 1 would let through.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::{Op, Vm};

/** The changes of `full`'s advice, past `n_inputs`, that `ops` accepts. */
pub(crate) fn unpinned(ops: &[Op], full: &[Fp], n_inputs: usize, case: usize) -> Vec<String> {
    let mut out = Vec::new();
    let two = Fp::from_u64(2);
    for idx in n_inputs..full.len() {
        let mut changes = vec![vec![(idx, Fp::ONE)]];
        if idx + 1 < full.len() {
            changes.push(vec![(idx, two), (idx + 1, -Fp::ONE)]);
            changes.push(vec![(idx, -two), (idx + 1, Fp::ONE)]);
        }
        for change in changes {
            let mut bad = full.to_vec();
            for &(i, d) in &change {
                bad[i] = bad[i] + d;
            }
            if Vm::new().run(ops, &bad, n_inputs).is_ok() {
                out.push(format!(
                    "case {case}: the advice at {idx} moves by {change:?}"
                ));
            }
        }
    }
    out
}
