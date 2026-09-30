/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Range-check removal. The program is walked in order, keeping each value's bound, which
 * the checks already passed tighten: after `RangeCheck(v, n)`, a bit of `v` below `2^n`,
 * a division of `n`-bit operands or `AssertBool(v)`, an accepted run has `v` in range. A
 * range check its value's bound already meets is dropped: it holds in every run that
 * reaches it.
 */

use alloc::vec::Vec;

use super::bounds::{def_bound, ANY};
use crate::compiler::ssa::{Builder, Inst, Ssa, V};

/** `ssa` without the range checks that always hold where they stand. */
pub fn elide_range_checks(ssa: &Ssa) -> Ssa {
    let mut bound: Vec<u128> = Vec::with_capacity(ssa.insts.len());
    let mut b = Builder::default();
    for (i, inst) in ssa.insts.iter().enumerate() {
        b.site = ssa.site(i);
        let get = |v: V| bound.get(v.index()).copied().unwrap_or(ANY);
        let insts = &ssa.insts;
        let k = |v: V| match insts.get(v.index()) {
            Some(Inst::Const(c)) => Some(*c),
            _ => None,
        };
        let own = def_bound(*inst, &get, &k);
        let limit = |n: u8| (1u128 << n.min(64)) - 1;
        let facts: [(Option<V>, u128); 2] = match *inst {
            Inst::RangeCheck(v, n) | Inst::Bit(v, _, n) => [(Some(v), limit(n)), (None, 0)],
            Inst::Quot(x, y, n) | Inst::Rem(x, y, n) => [(Some(x), limit(n)), (Some(y), limit(n))],
            Inst::AssertBool(v) => [(Some(v), 1), (None, 0)],
            _ => [(None, 0), (None, 0)],
        };
        let holds = matches!(*inst, Inst::RangeCheck(v, n) if get(v) <= limit(n));
        if !holds {
            b.emit(*inst);
        } else {
            b.emit(Inst::Const(0));
        }
        bound.push(own);
        for (v, lim) in facts {
            if let Some(slot) = v.and_then(|v| bound.get_mut(v.index())) {
                *slot = (*slot).min(lim);
            }
        }
    }
    Ssa {
        insts: b.ssa.insts,
        sites: b.ssa.sites,
        ..*ssa
    }
}
