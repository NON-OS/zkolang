/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checking a program: its items and imports, every constant and function, then the checks
 * over the whole program. The result is the typed program and every diagnostic; a
 * program with errors is not run.
 */

use super::cx::Sema;
use super::defs::{DefId, DefKind, Defs};
use crate::compiler::diag::Diagnostics;
use crate::compiler::syntax::ast::{Item, ItemKind, SourceAst};
use crate::compiler::tir::{ConstId, FnId, TProgram};

/** Check the program `ast`, a crate of one file. */
pub fn check(ast: &SourceAst) -> (TProgram, Diagnostics) {
    let mut sema = Sema::default();
    let (defs, imports) = Defs::collect(ast, &mut sema.diags);
    sema.defs = defs;
    sema.defs.resolve_imports(&imports, &mut sema.diags);
    sema.register();
    sema.impls(&ast.items);
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
    let program = sema.program();
    let diags = core::mem::take(&mut sema.diags);
    (program, diags)
}

impl<'a> Sema<'a> {
    /** Report every impl block of `items` and the modules inside them (E0904). */
    fn impls(&mut self, items: &'a [Item]) {
        for item in items {
            match &item.kind {
                ItemKind::Impl(_) => self.not_yet(item),
                ItemKind::Mod(m) => {
                    if let Some(body) = &m.body {
                        self.impls(body);
                    }
                }
                _ => {}
            }
        }
    }
}
