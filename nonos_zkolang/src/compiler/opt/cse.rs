/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Value numbering: an instruction equal to an earlier one, operands in a canonical order
 * for the commutative ones, is that one. Every instruction is a function of its operands,
 * a hint included, so the earlier one's value is the later one's; a repeated constraint
 * or output is dropped, since it holds or fails with the first. Advice values are the
 * exception: each stays its own, read where it is needed.
 */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::compiler::ssa::{Builder, Inst, Ssa, V};

/** `ssa` with each instruction that repeats an earlier one replaced by it. */
pub fn cse(ssa: &Ssa) -> Ssa {
    let mut b = Builder::default();
    let mut map: Vec<V> = Vec::with_capacity(ssa.insts.len());
    let mut seen: BTreeMap<Inst, V> = BTreeMap::new();
    for (i, inst) in ssa.insts.iter().enumerate() {
        b.site = ssa.site(i);
        let inst = canonical(inst.map(&mut |v| map.get(v.index()).copied().unwrap_or(v)));
        /* An advice value is kept apart from an equal one, since one read of it would be
         * held in a register from the first place it is needed to the last. */
        let apart = matches!(inst, Inst::Advice(_));
        let v = match seen.get(&inst).filter(|_| !apart) {
            Some(&v) => v,
            None => {
                let v = b.emit(inst);
                seen.insert(inst, v);
                v
            }
        };
        map.push(v);
    }
    Ssa {
        insts: b.ssa.insts,
        sites: b.ssa.sites,
        ..*ssa
    }
}

/** `inst` with the operands of a commutative operation in increasing order. */
fn canonical(inst: Inst) -> Inst {
    match inst {
        Inst::Add(a, b) if b < a => Inst::Add(b, a),
        Inst::Mul(a, b) if b < a => Inst::Mul(b, a),
        Inst::Eq(a, b) if b < a => Inst::Eq(b, a),
        _ => inst,
    }
}
