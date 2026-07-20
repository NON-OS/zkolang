/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The zKolang source language front-end: the lexer, the parser, the include
//! expander, and the compiler that lowers the tree to the VM instruction set the step
//! AIR proves. The language is small, total, and straight-line, so a program's step
//! count is a static property.

mod compile;
mod diagnostic;
mod error;
mod include;
mod lex;
mod optimize;
mod parse;

pub use compile::{compile, compile_full, compile_unoptimized, Compiled};
pub use diagnostic::render as render_error;
pub use error::CompileError;
pub use include::expand_includes;

use crate::isa::Op;
use alloc::vec::Vec;

/// Compile zKolang source into a VM program ending in `Halt`, ready for the VM to run
/// and the step AIR to prove.
pub fn compile_source(src: &str) -> Result<Vec<Op>, CompileError> {
    let (tokens, spans) = lex::lex(src)?;
    let ast = parse::parse(&tokens, &spans, src.len())?;
    compile(&ast)
}

/// Compile zKolang source into a VM program together with the advice plan its ordered
/// comparisons need, which the driver fills before proving.
pub fn compile_source_full(src: &str) -> Result<Compiled, CompileError> {
    let (tokens, spans) = lex::lex(src)?;
    let ast = parse::parse(&tokens, &spans, src.len())?;
    compile_full(&ast)
}

/// Compile zKolang source without the optimizer, for checking that optimization preserves
/// behavior against the optimized program.
pub fn compile_source_unoptimized(src: &str) -> Result<Vec<Op>, CompileError> {
    let (tokens, spans) = lex::lex(src)?;
    let ast = parse::parse(&tokens, &spans, src.len())?;
    compile_unoptimized(&ast)
}
