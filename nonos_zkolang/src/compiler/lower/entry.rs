/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Lowering a program (section 12): `main`'s parameters read from the inputs, the public
 * ones first, each slot checked against its type's invariant (section 12.3); the call of
 * `main` inlined; each slot of its result an output. A test is lowered the same way.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use super::sites::Sites;
use crate::compiler::ssa::{Builder, Inst, Ssa, V};
use crate::compiler::tir::{FnId, TProgram};

/** The SSA program of `p`'s `main`. */
pub fn lower_program(p: &TProgram) -> L<Ssa> {
    lower_function(p, p.main.ok_or(LowerError::NoMain)?)
}

/** The SSA program that runs `main` of `p`, its unlabelled parameters public. */
pub fn lower_function(p: &TProgram, main: FnId) -> L<Ssa> {
    lower_sited(p, main).map(|(ssa, _)| ssa)
}

/** `lower_function`, with the sites its instructions carry (section 15.2). */
pub fn lower_sited(p: &TProgram, main: FnId) -> L<(Ssa, Sites)> {
    let f = p.fns.get(main.0 as usize).ok_or(LowerError::NoMain)?;
    let mut lw = Lower {
        p,
        b: Builder::default(),
        g: V(0),
        frames: Vec::new(),
        sites: Sites::default(),
    };
    lw.g = lw.b.konst(1);
    lw.enter(f.span);
    let (args, n_public, next) = lw.read_params(f);
    let (result, _) = lw.inline(main, args)?;
    for (i, v) in result.iter().enumerate() {
        lw.b.emit(Inst::Output(i as u16, *v));
    }
    if lw.b.over {
        return Err(LowerError::TooLarge);
    }
    let mut ssa = lw.b.ssa;
    ssa.n_public = n_public;
    ssa.n_secret = next.saturating_sub(n_public);
    ssa.n_outputs = u16::try_from(result.len()).map_err(|_| LowerError::TooLarge)?;
    Ok((ssa, lw.sites))
}
