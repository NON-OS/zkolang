/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checking a program: its attributes, items and imports, every constant and function,
 * then the checks over the whole program. The result is the typed program and every
 * diagnostic; a program with errors is not run.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::collect::check_mode;
use crate::compiler::diag::Diagnostics;
use crate::compiler::package::CrateSrc;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::ast::SourceAst;
use crate::compiler::tir::TProgram;

/** A crate to check: its name, its syntax, and its dependencies by name and index. */
pub(super) struct CrateRef<'a> {
    pub(super) name: &'a str,
    pub(super) ast: &'a SourceAst,
    pub(super) deps: &'a [(String, usize)],
}

/**
 * Check the program `ast`, a crate whose modules are inline or loaded (`syntax::load`),
 * beside the standard library, which is loaded into `map`.
 */
pub fn check(map: &mut SourceMap, ast: &SourceAst) -> (TProgram, Diagnostics) {
    let only = CrateRef {
        name: "crate",
        ast,
        deps: &[],
    };
    check_mode(map, &[only], false)
}

/** Check `ast` for its tests: items marked `#[cfg(test)]` are compiled too. */
pub fn check_tests(map: &mut SourceMap, ast: &SourceAst) -> (TProgram, Diagnostics) {
    let only = CrateRef {
        name: "crate",
        ast,
        deps: &[],
    };
    check_mode(map, &[only], true)
}

/** Check `crates`, the program's crate first and the crates it depends on after; for its tests if `testing`. */
pub fn check_crates(
    map: &mut SourceMap,
    crates: &[CrateSrc],
    testing: bool,
) -> (TProgram, Diagnostics) {
    let refs: Vec<CrateRef> = crates
        .iter()
        .map(|c| CrateRef {
            name: &c.name,
            ast: &c.ast,
            deps: &c.deps,
        })
        .collect();
    check_mode(map, &refs, testing)
}
