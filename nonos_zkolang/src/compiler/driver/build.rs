/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Building an edition 2026 program from source: load its files, check, lower, compile and
 * verify, every failure a diagnostic. A built program carries its checked form, for the
 * reference run, and its ABI.
 */

use alloc::format;
use alloc::string::String;

use super::abi::abi_of;
use super::backend::backend;
use super::build_diag::{backend_failure, one, too_large};
use super::build_lower::lowering;
use super::built::Built;
use crate::compiler::diag::Diagnostics;
use crate::compiler::lower::lower_program;
use crate::compiler::sema::check;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::load::{load, Files};

/** The most rows a provable trace holds (section 15.1). */
pub const MAX_ROWS: usize = 1 << 16;

/**
 * Build the crate whose root file is `root`, a path and its text, reading its modules'
 * files from `files` into `map`: the program, or why not.
 */
pub fn build(
    map: &mut SourceMap,
    files: &dyn Files,
    root: (&str, String),
) -> Result<Built, Diagnostics> {
    let mut diags = Diagnostics::new();
    let ast = load(files, root, map, &mut diags);
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
