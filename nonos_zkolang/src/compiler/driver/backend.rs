/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * From SSA to a verified machine program: simplify, expand division, drop the range
 * checks that facts imply, simplify, expand bits, simplify, schedule, allocate and emit,
 * pad to the rows a hiding proof needs, then replay the result against the program it was
 * emitted from.
 */

use crate::compiler::codegen::{codegen, pad, verify, CodegenError, Machine, VerifyError};
use crate::compiler::gadget::{expand_bits, expand_division};
use crate::compiler::opt::{elide_range_checks, simplify};
use crate::compiler::schedule::schedule;
use crate::compiler::ssa::Ssa;

/**
 * The fewest rows a compiled program has: a trace of 2^5 rows is too short to be proved
 * hiding its witness, so every program is padded past it.
 */
pub const MIN_ROWS: usize = 33;

/** A compiled program: its machine code, and the machine-level SSA its witness runs. */
#[derive(Clone, Debug)]
pub struct Compiled {
    pub machine: Machine,
    pub ssa: Ssa,
}

/** Why the back end stopped. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackendError {
    Codegen(CodegenError),
    /** The emitted program does not implement the SSA program: a compiler bug. */
    Verify(VerifyError),
}

/** Compile `ssa` to a verified machine program. */
pub fn backend(ssa: &Ssa) -> Result<Compiled, BackendError> {
    let ssa = simplify(ssa);
    let ssa = simplify(&elide_range_checks(&expand_division(&ssa)));
    let ssa = schedule(&simplify(&expand_bits(&ssa)));
    let mut machine = codegen(&ssa).map_err(BackendError::Codegen)?;
    pad(&mut machine, MIN_ROWS);
    verify(&ssa, &machine).map_err(BackendError::Verify)?;
    Ok(Compiled { machine, ssa })
}
