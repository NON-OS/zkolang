/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Calls (sections 7.10 and 10.3): arguments left to right, a fresh frame for the callee,
 * and on return each `&mut` parameter's final value written back to its place.
 */

use alloc::vec::Vec;

use super::eval_place::Path;
use super::machine::{fail, Eval, Interp};
use super::FailKind;
use crate::compiler::source::Span;
use crate::compiler::tir::{FnId, TArg};

impl<'e> Interp<'e> {
    /** Call `f` on `args`, written at `at`. */
    pub(super) fn eval_call(&mut self, f: FnId, args: &[TArg], at: Span) -> Eval {
        let mut values = Vec::with_capacity(args.len());
        let mut places: Vec<Option<Path>> = Vec::with_capacity(args.len());
        for a in args {
            match a {
                TArg::Value(e) => {
                    values.push(self.eval(e)?);
                    places.push(None);
                }
                TArg::Place(p) => {
                    let path = self.path(p)?;
                    values.push(
                        self.read(&path)
                            .cloned()
                            .ok_or_else(|| fail(FailKind::Internal, p.span))?,
                    );
                    places.push(Some(path));
                }
            }
        }
        self.enter(at)?;
        let out = self.call_values(f, values, at);
        self.leave();
        let (result, finals) = out?;
        for (path, v) in places.into_iter().zip(finals) {
            if let Some(path) = path {
                self.write(&path, v);
            }
        }
        Ok(result)
    }
}
