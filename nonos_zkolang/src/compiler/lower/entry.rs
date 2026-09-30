/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Lowering a program (section 12): `main`'s parameters read from the inputs, the public
 * ones first, each slot checked against its type's invariant (section 12.3); the call of
 * `main` inlined; each slot of its result an output. A test is lowered the same way.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use super::layout::slots;
use crate::compiler::ssa::{Builder, Inst, Ssa, V};
use crate::compiler::syntax::ast::Label;
use crate::compiler::tir::{FnId, TProgram};

/** The SSA program of `p`'s `main`. */
pub fn lower_program(p: &TProgram) -> L<Ssa> {
    lower_function(p, p.main.ok_or(LowerError::NoMain)?)
}

/** The SSA program that runs `main` of `p`, its unlabelled parameters public. */
pub fn lower_function(p: &TProgram, main: FnId) -> L<Ssa> {
    let f = p.fns.get(main.0 as usize).ok_or(LowerError::NoMain)?;
    let mut lw = Lower {
        p,
        b: Builder::default(),
        g: V(0),
        frames: Vec::new(),
    };
    lw.g = lw.b.konst(1);
    let mut args: Vec<Vec<V>> = vec![Vec::new(); f.params.len()];
    let mut next: u16 = 0;
    let mut n_public = 0;
    for label in [Label::Public, Label::Secret] {
        for (i, param) in f.params.iter().enumerate() {
            let Some(local) = f.locals.get(param.local.0 as usize) else {
                continue;
            };
            if local.labels.whole().unwrap_or(Label::Public) != label {
                continue;
            }
            let n = slots(&p.types, local.ty);
            let vals: Vec<V> = (0..n)
                .map(|k| lw.b.emit(Inst::Input(next.saturating_add(k as u16))))
                .collect();
            next = next.saturating_add(n as u16);
            lw.check_input(local.ty, &vals);
            args[i] = vals;
        }
        if label == Label::Public {
            n_public = next;
        }
    }
    let (result, _) = lw.inline(main, args)?;
    for (i, v) in result.iter().enumerate() {
        lw.b.emit(Inst::Output(i as u16, *v));
    }
    if lw.b.over {
        return Err(LowerError::TooLarge);
    }
    let mut ssa = lw.b.ssa;
    ssa.n_public = n_public;
    ssa.n_secret = next - n_public;
    ssa.n_outputs = u16::try_from(result.len()).map_err(|_| LowerError::TooLarge)?;
    Ok(ssa)
}
