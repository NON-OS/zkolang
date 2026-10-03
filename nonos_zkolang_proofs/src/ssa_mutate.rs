/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Changing an accepted witness's advice, which the VM must then reject: each value by
 * one, and each two values one or two slots apart by `+2, -1`, `-2, +1`, `+1, -2` or
 * `-1, +2`. Those keep `2 x + y` or `x + 2 y`, the change a decomposition whose bits are
 * not constrained to 0 or 1 would let through, read from the bottom or the top, alone or
 * in lockstep with another.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::{Op, Vm};

/** The pairs of changes tried at two slots. */
const PAIRS: [(u64, u64, bool); 4] = [(2, 1, false), (2, 1, true), (1, 2, false), (1, 2, true)];

/** The changes of `full`'s advice, past `n_inputs`, that `ops` accepts. */
pub(crate) fn unpinned(ops: &[Op], full: &[Fp], n_inputs: usize, case: usize) -> Vec<String> {
    let mut out = Vec::new();
    for idx in n_inputs..full.len() {
        let mut changes = vec![vec![(idx, Fp::ONE)]];
        for stride in 1..=2 {
            if idx + stride < full.len() {
                for (x, y, down) in PAIRS {
                    let (x, y) = (Fp::from_u64(x), Fp::from_u64(y));
                    let (x, y) = if down { (-x, y) } else { (x, -y) };
                    changes.push(vec![(idx, x), (idx + stride, y)]);
                }
            }
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
