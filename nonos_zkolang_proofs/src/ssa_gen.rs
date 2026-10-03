/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Random SSA programs over two field elements and a range-checked 8-bit and 32-bit value.
 * Values are pooled by what they are known to be (a field element, a boolean, a value
 * below `2^n`), so most runs accept; some output many values, beyond the 32 registers.
 */

use nonos_zkolang::compiler::ssa::{Builder, Inst, Ssa, V};

use crate::prop_gen::Rng;

/** The pools: field elements, booleans, and values below 2^8, 2^16 and 2^32. */
pub(crate) struct Pools {
    pub field: Vec<V>,
    pub bools: Vec<V>,
    pub small: [(u8, Vec<V>); 3],
}

/** A random program of about `len` instructions. */
pub(crate) fn program(r: &mut Rng, len: usize) -> Ssa {
    let mut b = Builder::default();
    let inputs: Vec<V> = (0..4).map(|i| b.emit(Inst::Input(i))).collect();
    b.emit(Inst::RangeCheck(inputs[2], 8));
    b.emit(Inst::RangeCheck(inputs[3], 32));
    let mut p = Pools {
        field: vec![inputs[0], inputs[1]],
        bools: vec![b.konst(0), b.konst(1)],
        small: [
            (8, vec![inputs[2]]),
            (16, vec![b.konst(300)]),
            (32, vec![inputs[3]]),
        ],
    };
    for _ in 0..len {
        crate::ssa_gen_step::step(r, &mut b, &mut p);
    }
    let mut outs: Vec<V> = Vec::new();
    let wide = r.below(4) == 0;
    for (i, pool) in [&p.field, &p.bools, &p.small[0].1, &p.small[2].1]
        .iter()
        .enumerate()
    {
        let take = match (wide, i) {
            (true, 0) => 48,
            (true, _) => 12,
            (false, _) => 2,
        };
        outs.extend(pool.iter().rev().take(take));
    }
    for (i, v) in outs.iter().enumerate() {
        b.emit(Inst::Output(i as u16, *v));
    }
    b.ssa.n_public = 4;
    b.ssa.n_outputs = outs.len() as u16;
    b.ssa
}

/** Inputs: two field elements, then an 8-bit and a 32-bit value, one in ten out of range. */
pub(crate) fn inputs(r: &mut Rng) -> [nonos_stark::field::Fp; 4] {
    let mut small = |bits: u32| {
        let v = r.next() & ((1u64 << bits) - 1);
        if r.below(10) == 0 {
            v + (1 << bits)
        } else {
            v
        }
    };
    let (a, b) = (small(8), small(32));
    let f = nonos_stark::field::Fp::from_u64;
    [f(r.next()), f(r.below(5)), f(a), f(b)]
}
