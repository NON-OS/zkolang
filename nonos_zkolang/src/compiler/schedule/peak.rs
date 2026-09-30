/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! How many values an order keeps in registers at once, at most. */

use alloc::vec;

use super::graph::Graph;

/**
 * The most values `order` keeps at once that cannot be brought back: each from where it
 * is placed to its last reader. A constant or public input, which can be, is not counted.
 */
pub(super) fn peak(g: &Graph, order: &[usize]) -> i64 {
    let n = order.len();
    let mut pos = vec![0usize; n];
    for (k, &i) in order.iter().enumerate() {
        if let Some(p) = pos.get_mut(i) {
            *p = k;
        }
    }
    let mut diff = vec![0i64; n + 1];
    for v in (0..n).filter(|&v| !g.recoverable[v]) {
        let last = g.users[v].iter().map(|&u| pos[u]).max();
        if let Some(last) = last.filter(|&l| l > pos[v]) {
            diff[pos[v]] += 1;
            diff[last] -= 1;
        }
    }
    let (mut live, mut most) = (0i64, 0i64);
    for d in diff {
        live += d;
        most = most.max(live);
    }
    most
}
