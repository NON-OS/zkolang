/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Building an edition 2026 program from source: lex, parse, check, lower, compile and
 * verify, every failure a diagnostic. A built program carries its checked form, for the
 * reference run, and its ABI.
 */

use alloc::format;
use alloc::vec::Vec;

use super::abi::{abi_of, Leaf};
use super::backend::{backend, Compiled};
use super::build_diag::{backend_failure, one, too_large};
use super::build_lower::lowering;
use crate::compiler::diag::Diagnostics;
use crate::compiler::lower::lower_program;
use crate::compiler::sema::check;
use crate::compiler::source::FileId;
use crate::compiler::syntax::lex::lex;
use crate::compiler::syntax::parse::parse_file;
use crate::compiler::tir::TProgram;

/** The most rows a provable trace holds (section 15.1). */
pub const MAX_ROWS: usize = 1 << 16;

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

/** Build the program `src`, the text of `file`: the program, or why not. */
pub fn build(file: FileId, src: &str) -> Result<Built, Diagnostics> {
    let mut diags = Diagnostics::new();
    let lexed = lex(file, src, &mut diags);
    let ast = parse_file(file, src, &lexed, &mut diags, &mut 0);
    if diags.has_errors() {
        return Err(diags);
    }
    let (program, more) = check(&ast);
    diags.extend(more);
    if diags.has_errors() {
        return Err(diags);
    }
    let at = ast.span;
    let ssa = lower_program(&program).map_err(|e| lowering(e, at))?;
    let compiled = backend(&ssa).map_err(|e| backend_failure(e, at))?;
    let rows = compiled.machine.ops.len();
    if rows > MAX_ROWS {
        let what = format!("the program compiles to {rows} rows, more than 2^16");
        return Err(one(too_large(&what, at)));
    }
    let (public, secret, output) = abi_of(&program);
    Ok(Built {
        compiled,
        program,
        public,
        secret,
        output,
        warnings: diags,
    })
}
