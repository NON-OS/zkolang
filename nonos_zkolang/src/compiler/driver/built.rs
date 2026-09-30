/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A built program: what `build` gives for a program that checks and compiles. */

use alloc::vec::Vec;

use super::abi::Leaf;
use super::backend::Compiled;
use crate::compiler::diag::Diagnostics;
use crate::compiler::tir::TProgram;

/**
 * A built program: machine code, checked form, the leaves of its inputs and result, and
 * the warnings of checking it.
 */
#[derive(Clone, Debug)]
pub struct Built {
    pub compiled: Compiled,
    pub program: TProgram,
    pub public: Vec<Leaf>,
    pub secret: Vec<Leaf>,
    pub output: Vec<Leaf>,
    pub warnings: Diagnostics,
}
