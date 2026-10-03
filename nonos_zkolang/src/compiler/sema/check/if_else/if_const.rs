/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Constant conditions (section 8.4). An `if` condition that is a constant expression is
 * settled and evaluated where it stands, and only the block it takes is checked. A
 * constant expression reads no local, so its variables are its own and settling it now
 * gives them the types the end of the body would. A condition whose evaluation fails is
 * left to the run, as any other condition is.
 */

use super::super::cx::FnCx;
use crate::compiler::interp::Value;
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The value of the condition `c`, whose checks deferred from `mark` on are its own, if
     * it is a constant expression that evaluates with no error since `errors`.
     */
    pub(super) fn const_cond(&mut self, c: &mut TExpr, mark: usize, errors: usize) -> Option<bool> {
        if self.sema.diags.error_count() != errors || self.sema.not_const(c).is_some() {
            return None;
        }
        let own = self.deferred.split_off(mark.min(self.deferred.len()));
        own.iter().for_each(|d| self.settle_cast(d));
        self.rewrite(c);
        own.into_iter().for_each(|d| self.run_deferred(d));
        if self.sema.diags.error_count() != errors {
            /* Reported: the body's rewrite must not check it again. */
            *c = self.error(c.span);
            return None;
        }
        match self.sema.try_const_eval(c, self.locals.len())? {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
}
