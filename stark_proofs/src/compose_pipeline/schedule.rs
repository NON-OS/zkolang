// NONOS Operating System (AGPL-3.0-or-later)
//! Laying a tape onto a strip. Multiplications are the witnessed events; adds,
//! subs, and constants fold into the linear combinations around them. A mul's
//! level is one past its deepest witnessed operand, a row holds one level, and
//! a value consumed below its next row rides a carry lane. The plan is a pure
//! function of the tape, so the emitted shape stays derived.

use super::replay::const_mask;
use super::tape::Node;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// One scheduled multiplication: which tape node, which row produces it, and
/// the last row that reads it (its liveness end).
#[derive(Clone, Copy, Debug)]
pub struct Slot {
    pub node: u32,
    pub row: usize,
    pub last_use: usize,
}

pub struct Plan {
    pub slots: Vec<Slot>,
    /// Rows in the strip: the deepest mul level.
    pub rows: usize,
    /// Products alive at each row boundary: the carry pressure the width
    /// budget must absorb on top of the always-live inputs.
    pub peak_carry: usize,
    pub n_inputs: usize,
}

/// Level of every node: inputs and constants at 0; adds and subs are
/// transparent (their level is their deepest operand's); a mul sits one past
/// its deepest operand, because its check must read witnessed values from at
/// most one row above.
pub(super) fn levels_of(tape: &[Node]) -> Vec<usize> {
    let c = const_mask(tape);
    let mut lv: Vec<usize> = Vec::with_capacity(tape.len());
    for (i, node) in tape.iter().enumerate() {
        let l = match node {
            Node::Input(_) | Node::Const(_) => 0,
            Node::Add(a, b) | Node::Sub(a, b) => {
                lv[*a as usize].max(lv[*b as usize])
            }
            // A constant side makes the product a scalar edge: transparent,
            // like the adds it folds into.
            Node::Mul(a, b) if c[*a as usize] || c[*b as usize] => {
                lv[*a as usize].max(lv[*b as usize])
            }
            Node::Mul(a, b) => 1 + lv[*a as usize].max(lv[*b as usize]),
        };
        let _ = i;
        lv.push(l);
    }
    lv
}

/// The mul nodes each node's value depends on through transparent ops: for a
/// mul or leaf, itself; for adds and subs, the union of both sides. Computed
/// as the set of witnessed producers a consumer's linear combination reads.
pub(super) fn producers_of(tape: &[Node]) -> Vec<Vec<u32>> {
    let c = const_mask(tape);
    let mut ps: Vec<Vec<u32>> = Vec::with_capacity(tape.len());
    for (i, node) in tape.iter().enumerate() {
        let p = match node {
            Node::Input(_) | Node::Const(_) => Vec::new(),
            // A scalar edge passes its variable side's producers through.
            Node::Mul(a, b) if c[*a as usize] => ps[*b as usize].clone(),
            Node::Mul(a, b) if c[*b as usize] => ps[*a as usize].clone(),
            Node::Mul(_, _) => alloc::vec![i as u32],
            Node::Add(a, b) | Node::Sub(a, b) => {
                let mut m = ps[*a as usize].clone();
                for x in &ps[*b as usize] {
                    if !m.contains(x) {
                        m.push(*x);
                    }
                }
                m
            }
        };
        ps.push(p);
    }
    ps
}

/// Schedule the tape: every mul gets a row (its level), and its liveness end
/// is the deepest row of any mul that consumes it. `outputs` extends liveness
/// to the strip's end for the values the out-pins read.
pub fn plan(tape: &[Node], outputs: &[u32]) -> Plan {
    let c = const_mask(tape);
    let witnessed = |i: usize| match &tape[i] {
        Node::Mul(a, b) => !c[*a as usize] && !c[*b as usize],
        _ => false,
    };
    let lv = levels_of(tape);
    let ps = producers_of(tape);
    let rows = (0..tape.len()).filter(|i| witnessed(*i)).map(|i| lv[i]).max().unwrap_or(0);

    let mut last_use: BTreeMap<u32, usize> = BTreeMap::new();
    for (i, node) in tape.iter().enumerate() {
        if let Node::Mul(a, b) = node {
            for side in [a, b] {
                for prod in &ps[*side as usize] {
                    let e = last_use.entry(*prod).or_insert(0);
                    *e = (*e).max(lv[i]);
                }
            }
        }
    }
    for out in outputs {
        for prod in &ps[*out as usize] {
            last_use.insert(*prod, rows);
        }
    }

    let mut slots = Vec::new();
    for i in 0..tape.len() {
        if witnessed(i) {
            let n = i as u32;
            slots.push(Slot {
                node: n,
                row: lv[i],
                last_use: *last_use.get(&n).unwrap_or(&lv[i]),
            });
        }
    }

    // Carry pressure at each boundary r: products born at or before r, still
    // read after r. The width budget must hold this beside the input lanes.
    let mut peak_carry = 0usize;
    for r in 1..=rows {
        let live = slots
            .iter()
            .filter(|s| s.row <= r && s.last_use > r)
            .count();
        peak_carry = peak_carry.max(live);
    }

    let n_inputs = tape
        .iter()
        .filter(|n| matches!(n, Node::Input(_)))
        .count();
    Plan { slots, rows, peak_carry, n_inputs }
}

/// The window-2 audit: every mul's witnessed operands sit at its own row or
/// one above once carries are in place, which holds by construction when its
/// producers' liveness spans reach its row. This asserts that invariant.
pub fn audit(tape: &[Node], p: &Plan) -> bool {
    let lv = levels_of(tape);
    let ps = producers_of(tape);
    let by_node: BTreeMap<u32, &Slot> =
        p.slots.iter().map(|s| (s.node, s)).collect();
    for s in &p.slots {
        if let Node::Mul(a, b) = &tape[s.node as usize] {
            for side in [a, b] {
                for prod in &ps[*side as usize] {
                    let sp = by_node[prod];
                    if !(sp.row < s.row && sp.last_use >= s.row) {
                        return false;
                    }
                    let _ = lv[*prod as usize];
                }
            }
        }
    }
    true
}
