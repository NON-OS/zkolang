/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A tape cut to the operations a constraint reads, renumbered in order. */

use super::rec::Op;

/** The operations of `ops` that `outputs` read, and `outputs` renumbered onto them. */
pub(crate) fn prune(ops: &[Op], outputs: &[u32]) -> (Vec<Op>, Vec<u32>) {
    let mut live = vec![false; ops.len()];
    outputs.iter().for_each(|&o| live[o as usize] = true);
    for i in (0..ops.len()).rev() {
        if let (true, Op::Add(a, b) | Op::Sub(a, b) | Op::Mul(a, b)) = (live[i], ops[i]) {
            live[a as usize] = true;
            live[b as usize] = true;
        }
    }
    let mut at = vec![0u32; ops.len()];
    let mut kept = Vec::new();
    for (i, op) in ops.iter().enumerate() {
        if !live[i] {
            continue;
        }
        let m = |x: u32| at[x as usize];
        kept.push(match *op {
            Op::Add(a, b) => Op::Add(m(a), m(b)),
            Op::Sub(a, b) => Op::Sub(m(a), m(b)),
            Op::Mul(a, b) => Op::Mul(m(a), m(b)),
            other => other,
        });
        at[i] = kept.len() as u32 - 1;
    }
    (kept, outputs.iter().map(|&o| at[o as usize]).collect())
}
