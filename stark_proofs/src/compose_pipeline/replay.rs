// NONOS Operating System (AGPL-3.0-or-later)
//! Replaying a tape over concrete values: the check that the recording is
//! faithful before any strip is built from it. A tape that replays to the
//! same outputs as the direct evaluation, over random inputs, recorded the
//! computation and nothing else.

use super::tape::Node;
use crate::crypto::stark::field::Fp;

/// Evaluate every node over the base field. `inputs` are indexed by the
/// tape's input ordinals.
pub fn eval(tape: &[Node], inputs: &[Fp]) -> Vec<Fp> {
    let mut vals = Vec::with_capacity(tape.len());
    for node in tape {
        let v = match node {
            Node::Input(i) => inputs[*i as usize],
            Node::Const(c) => *c,
            Node::Add(a, b) => vals[*a as usize] + vals[*b as usize],
            Node::Sub(a, b) => vals[*a as usize] - vals[*b as usize],
            Node::Mul(a, b) => vals[*a as usize] * vals[*b as usize],
        };
        vals.push(v);
    }
    vals
}

/// The multiplication count: every one becomes a witnessed slot on the strip,
/// so this number is the strip's size before scheduling.
pub fn mul_count(tape: &[Node]) -> usize {
    tape.iter().filter(|n| matches!(n, Node::Mul(_, _))).count()
}

/// Whether each node is constant: a value with no input anywhere beneath it.
/// A constant-by-variable multiplication is a scalar edge in a linear
/// combination, not a witnessed product; only variable-by-variable products
/// cost strip slots.
pub fn const_mask(tape: &[Node]) -> Vec<bool> {
    let mut c: Vec<bool> = Vec::with_capacity(tape.len());
    for node in tape {
        let is = match node {
            Node::Const(_) => true,
            Node::Input(_) => false,
            Node::Add(a, b) | Node::Sub(a, b) | Node::Mul(a, b) => {
                c[*a as usize] && c[*b as usize]
            }
        };
        c.push(is);
    }
    c
}

/// The witnessed products: variable-by-variable multiplications only.
pub fn witnessed_muls(tape: &[Node]) -> Vec<u32> {
    let c = const_mask(tape);
    tape.iter()
        .enumerate()
        .filter_map(|(i, n)| match n {
            Node::Mul(a, b)
                if !c[*a as usize] && !c[*b as usize] =>
            {
                Some(i as u32)
            }
            _ => None,
        })
        .collect()
}
