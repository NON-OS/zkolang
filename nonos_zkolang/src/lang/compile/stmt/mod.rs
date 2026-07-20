/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Statement lowering, one statement kind per file. The dispatcher routes each
//! statement to its lowering: an array or scalar binding, an assertion, a public or
//! private input, an output, or the unrolled loop.

mod assert;
mod assert_eq;
mod assert_ne;
mod for_loop;
mod input;
mod let_array;
mod let_scalar;
mod output;
mod secret;

use super::compiler::Compiler;
use crate::lang::parse::{Expr, Stmt};
use crate::lang::CompileError;

impl Compiler {
    /// Lower one statement to its opcodes.
    pub(crate) fn stmt(&mut self, s: &Stmt) -> Result<(), CompileError> {
        match s {
            Stmt::Let(name, Expr::Array(elems)) => self.let_array(name, elems),
            Stmt::Let(name, e) => self.let_scalar(name, e),
            Stmt::Assert(e) => self.assert(e),
            Stmt::Input(name) => self.input(name),
            Stmt::Secret(name) => self.secret(name),
            Stmt::Output(e) => self.output(e),
            Stmt::For { var, lo, hi, body } => self.for_loop(var, *lo, *hi, body),
        }
    }
}
