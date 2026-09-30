/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The walk that rebuilds a program with some instructions expanded. */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::compiler::ssa::{Builder, Inst, Ssa, V};

/** A program being rebuilt, and where each old value went. */
pub(super) struct Rebuild {
    pub(super) b: Builder,
    pub(super) map: Vec<V>,
    /** The bits of each decomposition written so far, by value and width. */
    pub(super) bits: BTreeMap<(V, u8), Vec<V>>,
    /** The quotient and remainder of each division written so far. */
    pub(super) div: BTreeMap<(V, V, u8), (V, V)>,
}

/** Rebuild `ssa`, writing each instruction through `f`. */
fn rebuild(ssa: &Ssa, f: fn(&mut Rebuild, Inst) -> V) -> Ssa {
    let mut r = Rebuild {
        b: Builder::default(),
        map: Vec::with_capacity(ssa.insts.len()),
        bits: BTreeMap::new(),
        div: BTreeMap::new(),
    };
    for inst in &ssa.insts {
        let map = &r.map;
        let inst = inst.map(&mut |v| map.get(v.index()).copied().unwrap_or(v));
        let v = f(&mut r, inst);
        r.map.push(v);
    }
    Ssa {
        insts: r.b.ssa.insts,
        ..*ssa
    }
}

/** Expand division into advice, range checks and machine instructions. */
pub fn expand_division(ssa: &Ssa) -> Ssa {
    rebuild(ssa, |r, inst| match inst {
        Inst::Quot(a, b, n) => r.divide(a, b, n).0,
        Inst::Rem(a, b, n) => r.divide(a, b, n).1,
        _ => r.b.emit(inst),
    })
}

/** Expand every range check and bit into a bit decomposition. */
pub fn expand_bits(ssa: &Ssa) -> Ssa {
    rebuild(ssa, |r, inst| match inst {
        Inst::RangeCheck(v, n) => {
            r.decompose(v, n);
            v
        }
        Inst::Bit(v, k, n) => {
            let bit = r.decompose(v, n).get(usize::from(k)).copied();
            bit.unwrap_or_else(|| r.b.konst(0))
        }
        Inst::FieldBit(v, k) => {
            let bit = r.field_bits(v).get(usize::from(k)).copied();
            bit.unwrap_or_else(|| r.b.konst(0))
        }
        _ => r.b.emit(inst),
    })
}
