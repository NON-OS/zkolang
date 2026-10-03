/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Calls (section 10.4): every call is inlined. The callee runs under the caller's guard,
 * its `return`s select its result, and each `&mut` argument takes back the final value
 * of its parameter (section 10.3). The call ends on every path, so the guard after it is
 * the guard before.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::L;
use crate::compiler::ssa::V;
use crate::compiler::tir::{FnId, TArg};

impl<'p> Lower<'p> {
    /** A call of `f` on `args`; a `&mut` argument's place is evaluated once. */
    pub(super) fn call(&mut self, f: FnId, args: &[TArg]) -> L<Vec<V>> {
        let mut vals = Vec::with_capacity(args.len());
        let mut places = Vec::new();
        for a in args {
            vals.push(match a {
                TArg::Value(x) => self.expr(x)?,
                TArg::Place(p) => {
                    let steps = self.steps(p)?;
                    places.push((vals.len(), p.root, steps.clone()));
                    self.read_steps(p.root, &steps)
                }
            });
        }
        let (result, finals) = self.inline(f, vals)?;
        for (i, root, steps) in places {
            if let Some(fin) = finals.get(i) {
                self.write_steps(root, &steps, fin);
            }
        }
        Ok(result)
    }
}
