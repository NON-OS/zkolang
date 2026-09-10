// NONOS Operating System (AGPL-3.0-or-later)
//! The pressure-aware layout: schedule the products in DFS completion order
//! from the outputs, so each cone's intermediates die close to where they are
//! born, then pack that order into rows under a product budget. A value read
//! below the row after its birth rides a carry lane for the rows between.
//! The plan reports the width the budget actually buys, measured, so the
//! shape decision is a number.

use super::replay::const_mask;
use super::schedule::producers_of;
use super::tape::Node;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

pub struct Packed {
    /// Row of every mul node.
    pub rows_of: BTreeMap<u32, usize>,
    pub rows: usize,
    /// Peak simultaneous carry lanes (products crossing a row boundary).
    pub peak_carry: usize,
    /// Peak live input lanes at any boundary.
    pub peak_inputs: usize,
    /// Total base-column width: 2 per product slot, carry, and input lane.
    pub width: usize,
    /// Products the outputs consume linearly: absorbed by accumulator lanes
    /// the row they are born, so they cost accumulators, not carries.
    pub n_output_fed: usize,
    /// The real width driver under the cycle model: the most out-of-window
    /// values (products or inputs) any single row's constraints read.
    pub peak_echo: usize,
}

/// DFS completion order over the mul graph from the outputs: a cone's
/// products complete together, so liveness stays inside the cone.
fn cone_order(tape: &[Node], outputs: &[u32]) -> Vec<u32> {
    let ps = producers_of(tape);
    let c = const_mask(tape);
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    let mut order: Vec<u32> = Vec::new();
    let mut stack: Vec<(u32, bool)> = Vec::new();
    for out in outputs {
        for p in &ps[*out as usize] {
            stack.push((*p, false));
        }
        while let Some((n, expanded)) = stack.pop() {
            if seen.contains(&n) {
                continue;
            }
            if expanded {
                seen.insert(n);
                order.push(n);
                continue;
            }
            stack.push((n, true));
            if let Node::Mul(a, b) = &tape[n as usize] {
                if !c[*a as usize] && !c[*b as usize] {
                    for side in [a, b] {
                        for p in &ps[*side as usize] {
                            if !seen.contains(p) {
                                stack.push((*p, false));
                            }
                        }
                    }
                }
            }
        }
    }
    order
}

/// The input leaves reachable from `n` through transparent ops, iteratively:
/// the linear-combination chains run thousands of adds deep.
fn input_leaves(tape: &[Node], n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut stack = alloc::vec![n];
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    while let Some(x) = stack.pop() {
        if !seen.insert(x) {
            continue;
        }
        match &tape[x as usize] {
            Node::Input(_) => out.push(x),
            Node::Const(_) | Node::Mul(_, _) => {}
            Node::Add(a, b) | Node::Sub(a, b) => {
                stack.push(*a);
                stack.push(*b);
            }
        }
    }
    out
}

/// Pack `k` products per row in cone order and measure the pressure.
pub fn pack(tape: &[Node], outputs: &[u32], k: usize) -> Packed {
    let ps = producers_of(tape);
    let order = cone_order(tape, outputs);
    let mut rows_of: BTreeMap<u32, usize> = BTreeMap::new();
    for (idx, n) in order.iter().enumerate() {
        rows_of.insert(*n, idx / k);
    }
    let rows = order.len().div_ceil(k);

    let mut last_use: BTreeMap<u32, usize> = BTreeMap::new();
    let mut input_last: BTreeMap<u32, usize> = BTreeMap::new();
    for n in &order {
        let row = rows_of[n];
        if let Node::Mul(a, b) = &tape[*n as usize] {
            for side in [*a, *b] {
                for p in &ps[side as usize] {
                    let e = last_use.entry(*p).or_insert(0);
                    *e = (*e).max(row);
                }
                for inp in input_leaves(tape, side) {
                    let e = input_last.entry(inp).or_insert(0);
                    *e = (*e).max(row);
                }
            }
        }
    }
    // Products the outputs consume linearly do not carry: a running
    // accumulator lane absorbs each one the row it is born, so output
    // consumption extends no liveness. Only mul-operand reads did, above.
    let mut output_fed: BTreeSet<u32> = BTreeSet::new();
    for out in outputs {
        for p in &ps[*out as usize] {
            output_fed.insert(*p);
        }
    }

    let mut peak_carry = 0usize;
    let mut peak_inputs = 0usize;
    for r in 1..=rows {
        let carry = rows_of
            .iter()
            .filter(|(n, row)| **row < r && *last_use.get(n).unwrap_or(row) >= r)
            .count();
        let inputs = input_last.values().filter(|lu| **lu >= r).count();
        peak_carry = peak_carry.max(carry);
        peak_inputs = peak_inputs.max(inputs);
    }

    // Under the cycle model, width is not liveness: it is the distinct
    // out-of-window values each row's constraints read, every one an echo
    // cell filled by a copy cycle. Inputs count the same way.
    let mut peak_echo = 0usize;
    for r in 0..rows {
        let mut needed: BTreeSet<u32> = BTreeSet::new();
        for (n, row) in &rows_of {
            if *row != r {
                continue;
            }
            if let Node::Mul(a, b) = &tape[*n as usize] {
                for side in [*a, *b] {
                    for p in &ps[side as usize] {
                        let pr = rows_of[p];
                        if pr + 1 < r || pr > r {
                            needed.insert(*p);
                        }
                    }
                    for inp in input_leaves(tape, side) {
                        needed.insert(inp);
                    }
                }
            }
        }
        peak_echo = peak_echo.max(needed.len());
    }
    let width = 2 * (k + peak_echo);
    Packed { rows_of, rows, peak_carry, peak_inputs, width, n_output_fed: output_fed.len(), peak_echo }
}
