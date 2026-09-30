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
    /**
     * The bits of each decomposition written so far, by value and width, and where in the
     * old program it was written.
     */
    pub(super) bits: BTreeMap<(V, u8), (Vec<V>, usize)>,
    /** Where in the old program the walk is. */
    pub(super) at: usize,
    /** The quotient and remainder of each division written so far. */
    pub(super) div: BTreeMap<(V, V, u8), (V, V)>,
}

/** Rebuild `ssa`, writing each instruction through `f`. */
pub(super) fn rebuild(ssa: &Ssa, f: fn(&mut Rebuild, Inst) -> V) -> Ssa {
    let mut r = Rebuild {
        b: Builder::default(),
        map: Vec::with_capacity(ssa.insts.len()),
        bits: BTreeMap::new(),
        div: BTreeMap::new(),
        at: 0,
    };
    for (at, inst) in ssa.insts.iter().enumerate() {
        r.at = at;
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
