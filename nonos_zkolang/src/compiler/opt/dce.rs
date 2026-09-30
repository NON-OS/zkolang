/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Dead-code removal. An instruction stays if it constrains or outputs, if it may fail the
 * run (an inverse, a selection on a value not known boolean, a bit or a division), or if
 * one that stays reads it, its hint included.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::bools::booleans;
use crate::compiler::ssa::{Builder, Inst, Ssa, V};

/** `ssa` without the instructions nothing needs. */
pub fn dce(ssa: &Ssa) -> Ssa {
    let bools = booleans(ssa);
    let n = ssa.insts.len();
    let mut live = vec![false; n];
    for i in (0..n).rev() {
        let Some(inst) = ssa.insts.get(i) else {
            continue;
        };
        let may_fail = match *inst {
            Inst::Inv(a) => !matches!(ssa.get(a), Some(Inst::Const(c)) if *c != 0),
            Inst::Sel(c, _, _) => !bools.get(c.index()).copied().unwrap_or(false),
            Inst::Bit(..) | Inst::Quot(..) | Inst::Rem(..) => true,
            _ => inst.is_effect(),
        };
        if may_fail {
            live[i] = true;
        }
        if live.get(i).copied().unwrap_or(false) {
            for v in inst.operands().chain(inst.hint_operands()) {
                if let Some(l) = live.get_mut(v.index()) {
                    *l = true;
                }
            }
        }
    }
    let mut b = Builder::default();
    let mut map: Vec<V> = Vec::with_capacity(n);
    for (i, (inst, keep)) in ssa.insts.iter().zip(&live).enumerate() {
        b.site = ssa.site(i);
        let v = match keep {
            true => b.emit(inst.map(&mut |v| map.get(v.index()).copied().unwrap_or(v))),
            false => V(u32::MAX),
        };
        map.push(v);
    }
    Ssa {
        insts: b.ssa.insts,
        sites: b.ssa.sites,
        ..*ssa
    }
}
