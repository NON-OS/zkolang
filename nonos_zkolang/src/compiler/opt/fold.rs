/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Constant folding and algebraic identities, in one forward sweep. An operation on
 * constants becomes its value, unless it fails the run, which stays for the run to fail;
 * a constraint that holds of constants is dropped.
 */

use alloc::vec::Vec;

use super::fold_rules::{rule, Folded};
use crate::compiler::ssa::{Builder, Inst, Ssa, V};

/** `ssa` with every foldable instruction folded. */
pub fn fold(ssa: &Ssa) -> Ssa {
    let mut b = Builder::default();
    let mut map: Vec<V> = Vec::with_capacity(ssa.insts.len());
    for (i, inst) in ssa.insts.iter().enumerate() {
        b.site = ssa.site(i);
        let inst = inst.map(&mut |v| map.get(v.index()).copied().unwrap_or(v));
        let insts = &b.ssa.insts;
        let def = |v: V| insts.get(v.index()).copied();
        let v = match rule(inst, &def) {
            Folded::Keep => b.emit(inst),
            Folded::Value(v) => v,
            Folded::Const(c) => b.emit(Inst::Const(c)),
            Folded::Drop => V(u32::MAX),
        };
        map.push(v);
    }
    Ssa {
        insts: b.ssa.insts,
        sites: b.ssa.sites,
        ..*ssa
    }
}
