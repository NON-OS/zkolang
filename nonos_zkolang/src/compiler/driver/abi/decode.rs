/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Slots back to leaf values. */

use alloc::vec::Vec;

use nonos_stark::field::P;

use super::leaf::{slots, Leaf};

/**
 * The values of `data`, the slots of values whose leaves are `leaves`; `None` when their
 * count is not the count the leaves take or a slot holds no value of its leaf's type.
 */
pub fn decode(leaves: &[Leaf], data: &[u64]) -> Option<Vec<i128>> {
    if data.len() != slots(leaves) {
        return None;
    }
    let mut it = data.iter().map(|&x| i128::from(x));
    let mut out = Vec::with_capacity(leaves.len());
    for leaf in leaves {
        let x = it.next()?;
        let v = match leaf {
            Leaf::Int(t) if leaf.width() == 2 => {
                let high = it.next()?;
                let half = 0..1i128 << 32;
                if !half.contains(&x) || !half.contains(&high) {
                    return None;
                }
                let pattern = x | high << 32;
                if t.signed() && pattern >= 1i128 << 63 {
                    pattern - (1i128 << 64)
                } else {
                    pattern
                }
            }
            Leaf::Int(t) if t.signed() && x > i128::from(P) / 2 => x - i128::from(P),
            _ => x,
        };
        let fits = match leaf {
            Leaf::Bool => v == 0 || v == 1,
            Leaf::Field => true,
            Leaf::Int(t) => t.contains(v),
        };
        out.push(fits.then_some(v)?);
    }
    Some(out)
}
