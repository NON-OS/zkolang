/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Running a program in SSA form over field elements. It is the semantics every pass keeps
 * and every gadget expansion implements, and, run on the program that is emitted, the
 * witness generator: each hint finds its advice from the values before it.
 */

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::{Inst, Ssa, V};

/** A run that fails: the instruction at `at` does not hold. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SsaFailure {
    pub at: V,
}

/** An accepted run: every instruction's value, the outputs, and the advice in order. */
#[derive(Clone, Debug)]
pub struct SsaRun {
    pub values: Vec<Fp>,
    pub outputs: Vec<Fp>,
    pub advice: Vec<Fp>,
}

/** Run `ssa` on `inputs`, the public slots then the secret ones. */
pub fn eval(ssa: &Ssa, inputs: &[Fp]) -> Result<SsaRun, SsaFailure> {
    let mut run = SsaRun {
        values: Vec::with_capacity(ssa.insts.len()),
        outputs: vec![Fp::ZERO; usize::from(ssa.n_outputs)],
        advice: Vec::new(),
    };
    for (i, inst) in ssa.insts.iter().enumerate() {
        let at = V(u32::try_from(i).unwrap_or(u32::MAX));
        let v = step(*inst, inputs, &mut run).ok_or(SsaFailure { at })?;
        run.values.push(v);
    }
    Ok(run)
}

/** The value of `inst`, or `None` if it does not hold. */
fn step(inst: Inst, inputs: &[Fp], run: &mut SsaRun) -> Option<Fp> {
    let get = |v: V| run.values.get(v.index()).copied();
    Some(match inst {
        Inst::Const(c) => Fp::from_u64(c),
        Inst::Input(i) => *inputs.get(usize::from(i))?,
        Inst::Advice(h) => {
            let v = super::eval_hint::hint(h, &run.values)?;
            run.advice.push(v);
            v
        }
        Inst::Add(a, b) => get(a)? + get(b)?,
        Inst::Sub(a, b) => get(a)? - get(b)?,
        Inst::Mul(a, b) => get(a)? * get(b)?,
        Inst::Inv(a) => Some(get(a)?).filter(|x| *x != Fp::ZERO)?.inv(),
        Inst::Sel(c, a, b) => match get(c)?.value() {
            1 => get(a)?,
            0 => get(b)?,
            _ => return None,
        },
        Inst::Eq(a, b) => Fp::from_u64(u64::from(get(a)? == get(b)?)),
        Inst::Output(i, a) => {
            *run.outputs.get_mut(usize::from(i))? = get(a)?;
            Fp::ZERO
        }
        _ => super::eval_check::check(inst, &run.values)?,
    })
}
