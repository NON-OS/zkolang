/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One pass over an expression: every node's id, and the sizes and counts sharing needs. */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::ids::Ids;
use super::pure::{children, hoistable};
use crate::lang::parse::Expr;

/** A subtree that may be shared: its id, its visible size, and the subtree itself. */
pub(super) struct Candidate<'a> {
    pub(super) id: usize,
    pub(super) size: usize,
    pub(super) expr: &'a Expr,
}

/**
 * Walk `e` bottom-up. Every visible hoistable node goes onto `order` in pre-order, every
 * visible node is counted under its id, and the return is `e`'s id and visible size.
 */
pub(super) fn scan<'a>(
    e: &'a Expr,
    ids: &mut Ids,
    order: &mut Vec<Candidate<'a>>,
    counts: &mut BTreeMap<usize, usize>,
) -> (usize, usize) {
    let slot = hoistable(e).then(|| {
        order.push(Candidate {
            id: 0,
            size: 0,
            expr: e,
        });
        order.len() - 1
    });
    let (mut kids, mut size) = (Vec::new(), 1);
    for c in children(e) {
        let (id, s) = scan(c, ids, order, counts);
        kids.push(id);
        size += s;
    }
    let id = ids.node(e, kids);
    *counts.entry(id).or_insert(0) += 1;
    if let Some(c) = slot.and_then(|i| order.get_mut(i)) {
        c.id = id;
        c.size = size;
    }
    (id, size)
}
