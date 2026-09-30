/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * From SSA to a verified machine program: simplify, expand division, drop the range
 * checks that facts imply, simplify, expand bits, simplify, allocate and emit, then
 * replay the result against the program it was emitted from.
 */

use crate::compiler::codegen::{codegen, verify, CodegenError, Machine, VerifyError};
use crate::compiler::gadget::{expand_bits, expand_division};
use crate::compiler::opt::{elide_range_checks, simplify};
use crate::compiler::ssa::Ssa;

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
    let ssa = simplify(&expand_bits(&ssa));
    let machine = codegen(&ssa).map_err(BackendError::Codegen)?;
    verify(&ssa, &machine).map_err(BackendError::Verify)?;
    Ok(Compiled { machine, ssa })
}
