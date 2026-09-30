/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checking a program: its attributes, items and imports, every constant and function,
 * then the checks over the whole program. The result is the typed program and every
 * diagnostic; a program with errors is not run.
 */

use super::cx::{Sema, State};
use super::defs::{DefId, DefKind, Defs};
use super::lints::drop_allowed;
use crate::compiler::diag::Diagnostics;
use crate::compiler::syntax::ast::SourceAst;
use crate::compiler::tir::{ConstId, FnId, TProgram};

/** Check the program `ast`, a crate of one file. */
pub fn check(ast: &SourceAst) -> (TProgram, Diagnostics) {
    check_mode(ast, false)
}

/** Check `ast` for its tests: items marked `#[cfg(test)]` are compiled too. */
pub fn check_tests(ast: &SourceAst) -> (TProgram, Diagnostics) {
    check_mode(ast, true)
}

fn check_mode(ast: &SourceAst, testing: bool) -> (TProgram, Diagnostics) {
    let mut sema = Sema::default();
    sema.check_attrs(&ast.items, &ast.inner_attrs, ast.span);
    let (defs, imports) = Defs::collect_with(ast, testing, &mut sema.diags);
    sema.defs = defs;
    sema.defs.resolve_imports(&imports, &mut sema.diags);
    sema.register();
    sema.impls(&ast.items);
    sema.check_main();
    sema.structs();
    for (i, d) in sema.defs.defs.clone().iter().enumerate() {
        if d.kind == DefKind::Alias {
            sema.alias(DefId(u32::try_from(i).unwrap_or(u32::MAX)));
        }
    }
    for c in 0..sema.consts.len() {
        sema.const_value(ConstId(u32::try_from(c).unwrap_or(u32::MAX)));
    }
    for f in 0..sema.fns.len() {
        sema.check_body(FnId(u32::try_from(f).unwrap_or(u32::MAX)));
    }
    sema.check_const_fns();
    sema.check_recursion();
    let mut program = sema.program();
    program.tests = sema.tests();
    let ok: alloc::vec::Vec<bool> = sema
        .fns
        .iter()
        .map(|f| matches!(f.body, State::Done(_)))
        .collect();
    let mut diags = core::mem::take(&mut sema.diags);
    crate::compiler::sema::secret::check_program(&program, &ok, &mut diags);
    (program, drop_allowed(diags, &sema.allowed))
}
