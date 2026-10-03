/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The checks after a program's items are collected and its imports resolved: `main`, the
 * types, the constants, every function body, recursion, and secret flow.
 */

use super::super::cx::{Sema, State};
use super::super::defs::{DefId, DefKind};
use super::super::lints::drop_allowed;
use crate::compiler::diag::Diagnostics;
use crate::compiler::tir::{ConstId, FnId, TProgram};

impl Sema<'_> {
    /** Run the checks left, and give the typed program with every diagnostic. */
    pub(super) fn finish(self) -> (TProgram, Diagnostics) {
        let mut sema = self;
        sema.check_main();
        sema.structs();
        for (i, d) in sema.defs.defs.clone().iter().enumerate() {
            let def = DefId(u32::try_from(i).unwrap_or(u32::MAX));
            if d.kind == DefKind::Alias && sema.params_of(def).is_empty() {
                sema.alias(def);
            }
        }
        for c in 0..sema.consts.len() {
            sema.const_value(ConstId(u32::try_from(c).unwrap_or(u32::MAX)));
        }
        let mut f = 0;
        while f < sema.fns.len() {
            sema.check_body(FnId(u32::try_from(f).unwrap_or(u32::MAX)));
            f += 1;
        }
        sema.check_const_fns();
        sema.check_idle_templates();
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
}
