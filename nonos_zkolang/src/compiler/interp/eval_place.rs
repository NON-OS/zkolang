/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Places (section 8.2): the path into a local is evaluated once, its indices checked, and
 * then the part it names is read or written.
 */

use alloc::vec::Vec;

use super::machine::{fail, Eval, Flow, Interp};
use super::{FailKind, Value};
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::tir::{Proj, TExpr, TPlace};

/** A place with its indices evaluated: the local and the steps into it. */
pub(super) struct Path {
    pub(super) root: u32,
    pub(super) steps: Vec<usize>,
}

impl<'e> Interp<'e> {
    /** Evaluate the indices of `p`, checking each is in bounds. */
    pub(super) fn path(&mut self, p: &TPlace) -> Result<Path, Flow> {
        let mut steps = Vec::with_capacity(p.proj.len());
        for proj in &p.proj {
            let at = match proj {
                Proj::TupleField(i) => *i as usize,
                Proj::Index(e) => {
                    let i = self.eval(e)?.int();
                    let len = self
                        .read(&Path {
                            root: p.root.0,
                            steps: steps.clone(),
                        })
                        .map_or(0, |v| v.parts().len());
                    match usize::try_from(i) {
                        Ok(i) if i < len => i,
                        _ => return Err(fail(FailKind::IndexOutOfBounds, e.span)),
                    }
                }
            };
            steps.push(at);
        }
        Ok(Path {
            root: p.root.0,
            steps,
        })
    }

    /** `place = value`, or `place op= value`. */
    pub(super) fn assign(&mut self, place: &TPlace, op: Option<BinOp>, value: &TExpr) -> Eval {
        let path = self.path(place)?;
        let rhs = self.eval(value)?;
        let new = match op {
            None => rhs,
            Some(op) => {
                let cur = self
                    .read(&path)
                    .cloned()
                    .ok_or_else(|| fail(FailKind::Internal, place.span))?;
                self.binop(op, &cur, &rhs, place.ty)
                    .map_err(|k| fail(k, value.span))?
            }
        };
        self.write(&path, new);
        Ok(Value::Unit)
    }
}
