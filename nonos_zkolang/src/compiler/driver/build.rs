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

use super::abi::abi_of;
use super::backend::backend;
use super::build_diag::{backend_failure, one, too_large};
use super::build_lower::lowering;
use super::built::Built;
use super::source::{crates_of, Source};
use crate::compiler::diag::Diagnostics;
use crate::compiler::lower::{lower_sited, LowerError};
use crate::compiler::sema::check_crates;
use crate::compiler::source::{SourceMap, Span};
use crate::compiler::syntax::load::Files;

/** The most rows a provable trace holds (section 15.1). */
pub const MAX_ROWS: usize = 1 << 16;

/**
 * Build the program `src`, reading its files from `files` into `map`: the program, or why
 * not.
 */
pub fn build(
    map: &mut SourceMap,
    files: &dyn Files,
    src: Source<'_>,
) -> Result<Built, Diagnostics> {
    let mut diags = Diagnostics::new();
    let crates = crates_of(files, src, map, &mut diags);
    if diags.has_errors() {
        return Err(diags);
    }
    let (program, more) = check_crates(map, &crates, false);
    diags.extend(more);
    if diags.has_errors() {
        return Err(diags);
    }
    let at = crates.first().map_or(Span::DUMMY, |c| c.ast.span);
    let main = program.main.ok_or(LowerError::NoMain);
    let lowered = main.and_then(|m| lower_sited(&program, m));
    let (ssa, sites) = lowered.map_err(|e| lowering(e, at))?;
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
        sites,
    })
}
