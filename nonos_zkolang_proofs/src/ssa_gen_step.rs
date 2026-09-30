/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One random instruction of a generated SSA program. */

use nonos_zkolang::compiler::ssa::{Builder, Inst, V};

use crate::prop_gen::Rng;
use crate::ssa_gen::Pools;

fn pick(r: &mut Rng, xs: &[V]) -> V {
    xs[r.below(xs.len() as u64) as usize]
}

/** Append one instruction and file its value in the pool it belongs to. */
pub(crate) fn step(r: &mut Rng, b: &mut Builder, p: &mut Pools) {
    let n = r.below(3) as usize;
    let (bits, ref smalls) = p.small[n];
    let (f1, f2) = (pick(r, &p.field), pick(r, &p.field));
    let (s1, s2) = (pick(r, smalls), pick(r, smalls));
    let c = pick(r, &p.bools);
    match r.below(14) {
        0 => p.field.push(b.add(f1, f2)),
        1 => p.field.push(b.sub(f1, f2)),
        2 => p.field.push(b.mul(f1, f2)),
        3 if r.below(4) == 0 => p.field.push(b.emit(Inst::Inv(f1))),
        4 => p.bools.push(b.emit(Inst::Eq(f1, f2))),
        5 => p.field.push(b.emit(Inst::Sel(c, f1, f2))),
        6 => {
            let k = r.below(u64::from(bits)) as u8;
            p.bools.push(b.emit(Inst::Bit(s1, k, bits)));
        }
        7 => {
            let one = b.konst(1);
            let d = b.add(s2, one);
            let d = if bits < 32 { d } else { s2 };
            let op = if r.below(2) == 0 {
                Inst::Quot(s1, d, bits)
            } else {
                Inst::Rem(s1, d, bits)
            };
            if bits < 32 {
                p.small[n].1.push(b.emit(op));
            } else {
                p.small[2].1.push(b.emit(op));
            }
        }
        8 => {
            b.emit(Inst::RangeCheck(s1, bits));
        }
        9 if n == 0 => p.small[1].1.push(b.mul(s1, s2)),
        10 => p.bools.push(b.emit(Inst::FieldBit(f1, r.below(64) as u8))),
        11 => {
            let nc = b.not(c);
            let z = b.mul(c, nc);
            b.emit(Inst::AssertZero(z));
        }
        12 => p.field.push(s1),
        _ => p.bools.push(b.mul(c, pick(r, &p.bools))),
    }
}
