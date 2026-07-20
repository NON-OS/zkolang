/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The zKolang source language front-end: the lexer, the parser, the include
//! expander, and the compiler that lowers the tree to the VM instruction set the step
//! AIR proves. The language is small, total, and straight-line, so a program's step
//! count is a static property.

mod compile;
mod error;
mod include;
mod lex;
mod parse;

pub use compile::{compile, compile_full, Compiled};
pub use error::CompileError;
pub use include::expand_includes;

use crate::isa::Op;
use alloc::vec::Vec;

/// Compile zKolang source into a VM program ending in `Halt`, ready for the VM to run
/// and the step AIR to prove.
pub fn compile_source(src: &str) -> Result<Vec<Op>, CompileError> {
    let tokens = lex::lex(src)?;
    let ast = parse::parse(&tokens)?;
    compile(&ast)
}

/// Compile zKolang source into a VM program together with the advice plan its ordered
/// comparisons need, which the driver fills before proving.
pub fn compile_source_full(src: &str) -> Result<Compiled, CompileError> {
    let tokens = lex::lex(src)?;
    let ast = parse::parse(&tokens)?;
    compile_full(&ast)
}
