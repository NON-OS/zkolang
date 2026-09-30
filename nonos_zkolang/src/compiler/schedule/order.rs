/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Two orders: the one the program was written in, and each sink's dependences depth
 * first. In both, a constant, input or advice value is placed only when something needs
 * it; a sink as soon as its operands are; and an operation that is the last to read one
 * of its operands as soon as its operands are, since it holds no more values than it frees.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::graph::Graph;

/** The walk: what is placed, in which order, and how many readers each value awaits. */
pub(super) struct Walk<'g> {
    pub(super) g: &'g Graph,
    pub(super) placed: Vec<bool>,
    pub(super) order: Vec<usize>,
    pub(super) left: Vec<usize>,
}

/** The start of a walk over the `n` instructions of `g`. */
pub(super) fn walk(g: &Graph, n: usize) -> Walk<'_> {
    Walk {
        g,
        placed: vec![false; n],
        order: Vec::with_capacity(n),
        left: g.users.iter().map(Vec::len).collect(),
    }
}

/** The instructions in the order they were written, each read put off until needed. */
pub(super) fn in_place(g: &Graph, n: usize) -> Vec<usize> {
    let mut w = walk(g, n);
    for i in (0..n).filter(|&i| g.op[i] || g.sink[i]) {
        w.visit(i);
    }
    for i in 0..n {
        w.visit(i);
    }
    w.order
}

/** Each sink's dependences depth first, the sinks in the order they were written. */
pub(super) fn depth_first(g: &Graph, n: usize) -> Vec<usize> {
    let mut w = walk(g, n);
    for (j, &s) in g.sinks.iter().enumerate() {
        /* A sink whose every operand a later sink needs is placed when they are ready. */
        if !g.deps[s].iter().all(|&d| g.last[d] > j) {
            w.visit(s);
        }
    }
    for s in 0..n {
        w.visit(s);
    }
    w.order
}
