/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checking a program without building it, so a crate with no `fn main`, a library, can be
 * checked: nothing in it runs, and so nothing in it is lowered.
 */

use super::source::{crates_of, Source};
use crate::compiler::diag::Diagnostics;
use crate::compiler::sema::check_crates;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::load::Files;

/**
 * Check the program `src`, reading its files from `files` into `map`: whether its root
 * module has a `main`, and its warnings; or the diagnostics of a program that does not
 * check.
 */
pub fn check(
    map: &mut SourceMap,
    files: &dyn Files,
    src: Source<'_>,
) -> Result<(bool, Diagnostics), Diagnostics> {
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
    Ok((program.main.is_some(), diags))
}
