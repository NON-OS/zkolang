/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The program rewritten in the scheduled order. */

use alloc::vec;
use alloc::vec::Vec;

use super::graph::Graph;
use super::order::{depth_first, in_place};
use super::peak::peak;
use crate::compiler::ssa::{Ssa, V};

/** `ssa` with its instructions reordered to keep few values in registers at once. */
pub fn schedule(ssa: &Ssa) -> Ssa {
    let n = ssa.insts.len();
    let g = Graph::of(ssa);
    /* Of the orders tried, the one keeping the fewest values at once. */
    let order = [in_place(&g, n), depth_first(&g, n)]
        .into_iter()
        .min_by_key(|o| peak(&g, o))
        .unwrap_or_default();
    let mut new = vec![V(u32::MAX); n];
    for (k, &i) in order.iter().enumerate() {
        new[i] = V(u32::try_from(k).unwrap_or(u32::MAX));
    }
    let insts: Vec<_> = order
        .iter()
        .map(|&i| ssa.insts[i].map(&mut |v| new.get(v.index()).copied().unwrap_or(v)))
        .collect();
    Ssa { insts, ..*ssa }
}
