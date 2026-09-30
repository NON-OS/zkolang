/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A `match` once every type is known (section 9.3): its patterns rewritten, then an arm no
 * value reaches warned of (W0004), and a value no arm covers reported (E0400). A `match`
 * whose scrutinee or patterns failed to check is not looked at, so one mistake is
 * reported once.
 */

use super::super::cx::FnCx;
use crate::compiler::sema::ty::Types;
use crate::compiler::tir::{TArm, TExpr};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Rewrite `match s { arms }`, and check that its arms cover every value. */
    pub(crate) fn rewrite_match(&mut self, s: &mut TExpr, arms: &mut [TArm]) {
        self.rewrite(s);
        let errors = self.sema.diags.error_count();
        arms.iter_mut()
            .for_each(|a| self.rewrite_pat(&mut a.pat, s.ty));
        let broken = self.sema.diags.error_count() > errors
            || arms.iter().any(|a| self.pats.broken.contains(&a.span));
        for a in arms.iter_mut() {
            if let Some(g) = &mut a.guard {
                self.rewrite(g);
            }
            self.rewrite(&mut a.body);
        }
        if !broken && s.ty != Types::ERROR {
            self.exhaustive(s, arms);
        }
    }
}
