/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A dishonest prover: a witness in which chosen advice values are replaced, every other
 * advice value then computed by its hint from the values as they now stand, so a check
 * that the replacement breaks is the only thing that can catch it.
 */

use std::collections::BTreeMap;

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::Compiled;
use nonos_zkolang::compiler::ssa::eval_hint::hint;
use nonos_zkolang::compiler::ssa::{Inst, V};

/** The witness of `c` on `inputs` with advice slot `s` set to `f(s, values)` when `Some`. */
pub(crate) fn forged(
    c: &Compiled,
    inputs: &[Fp],
    f: &dyn Fn(usize, &[Fp]) -> Option<Fp>,
) -> Vec<Fp> {
    let mut values: Vec<Fp> = Vec::new();
    let mut slots: BTreeMap<usize, Fp> = BTreeMap::new();
    for inst in &c.ssa.insts {
        let get = |v: V| values.get(v.0 as usize).copied().unwrap_or(Fp::ZERO);
        let x = match *inst {
            Inst::Const(k) => Fp::from_u64(k),
            Inst::Input(i) => inputs.get(usize::from(i)).copied().unwrap_or(Fp::ZERO),
            Inst::Advice(h) => {
                let s = slots.len();
                let x = f(s, &values).unwrap_or_else(|| hint(h, &values).unwrap_or(Fp::ZERO));
                slots.insert(s, x);
                x
            }
            Inst::Add(a, b) => get(a) + get(b),
            Inst::Sub(a, b) => get(a) - get(b),
            Inst::Mul(a, b) => get(a) * get(b),
            Inst::Inv(a) if get(a) == Fp::ZERO => Fp::ZERO,
            Inst::Inv(a) => get(a).inv(),
            Inst::Sel(k, a, b) => get(k) * (get(a) - get(b)) + get(b),
            Inst::Eq(a, b) => Fp::from_u64(u64::from(get(a) == get(b))),
            _ => Fp::ZERO,
        };
        values.push(x);
    }
    let mut full = inputs.to_vec();
    for (s, h) in c.machine.advice.iter().enumerate() {
        full.push(
            slots
                .get(&s)
                .copied()
                .unwrap_or_else(|| hint(*h, &values).unwrap_or(Fp::ZERO)),
        );
    }
    full
}
