/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Where each input slot of a program comes from. The public inputs and secrets come from
 * the caller. Past them sit the bits of each ordered comparison, which the prover fills
 * from the run; a native target computes each bit from the value the program decomposes,
 * which is written before the bits are read and held until they are recomposed.
 */

use alloc::collections::BTreeMap;

use crate::isa::Op;
use crate::lang::Compiled;

/** The input slots a native target asks its caller for, and the ones it computes. */
pub(crate) struct Plan {
    /** How many inputs the caller supplies: the public inputs, then the secrets. */
    pub(crate) n_user: usize,
    bits: BTreeMap<u32, (u8, u8)>,
}

impl Plan {
    pub(crate) fn of(compiled: &Compiled) -> Plan {
        let n_user = usize::from(compiled.n_public) + usize::from(compiled.n_secret);
        let mut bits = BTreeMap::new();
        for a in &compiled.advice {
            let Some(reg) = compiled.ops.get(a.value_op as usize).and_then(written) else {
                continue;
            };
            for k in 0..a.width {
                let slot = n_user as u32 + u32::from(a.start) + u32::from(k);
                bits.insert(slot, (reg, k));
            }
        }
        Plan { n_user, bits }
    }

    /** For a comparison bit, the register holding the value and which bit it is. */
    pub(crate) fn bit(&self, idx: u16) -> Option<(u8, u8)> {
        self.bits.get(&u32::from(idx)).copied()
    }
}

/** The register an instruction writes, if it writes one. */
fn written(op: &Op) -> Option<u8> {
    match op {
        Op::Imm { d, .. }
        | Op::Add { d, .. }
        | Op::Sub { d, .. }
        | Op::Mul { d, .. }
        | Op::Inv { d, .. }
        | Op::Sel { d, .. }
        | Op::Eq { d, .. }
        | Op::Inp { d, .. } => Some(*d),
        Op::Bool { .. } | Op::Assert { .. } | Op::Out { .. } | Op::Halt => None,
    }
}
