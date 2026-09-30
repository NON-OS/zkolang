/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Blocks and statements (section 8): `let` binds the slots of its value to the pattern's
 * locals, `assert` fails the run where it runs and its condition is false, and a block's
 * value is its tail's.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::L;
use super::layout::{elements, field_at, slots};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::ssa::V;
use crate::compiler::tir::{TBlock, TPat, TStmt};

impl<'p> Lower<'p> {
    /** The value of the block `b`. */
    pub(super) fn block(&mut self, b: &TBlock) -> L<Vec<V>> {
        for s in &b.stmts {
            match s {
                TStmt::Let { pat, init } => {
                    let v = self.expr(init)?;
                    self.bind(pat, &v, init.ty)?;
                }
                TStmt::Assert { cond, .. } => {
                    let c = self.expr(cond)?;
                    if let Some(&c) = c.first() {
                        self.require(c);
                    }
                }
                TStmt::Expr(e) => {
                    self.expr(e)?;
                }
            }
        }
        match &b.tail {
            Some(t) => self.expr(t),
            None => Ok(Vec::new()),
        }
    }

    /** Bind the locals of `pat` to the parts of `vals`, a value of type `t`. */
    pub(super) fn bind(&mut self, pat: &TPat, vals: &[V], t: TyId) -> L<()> {
        match pat {
            TPat::Bind(l) => self.set_local(*l, vals.to_vec()),
            TPat::Wild => {}
            TPat::Tuple(ps) => {
                let types = &self.p.types;
                let parts: Vec<(usize, TyId)> = match types.kind(t) {
                    TyKind::Array(..) => match elements(types, t) {
                        Some((e, size, _)) => (0..ps.len()).map(|k| (k * size, e)).collect(),
                        None => Vec::new(),
                    },
                    _ => (0..ps.len())
                        .filter_map(|k| field_at(types, t, k as u32))
                        .collect(),
                };
                for (p, (at, ty)) in ps.iter().zip(parts) {
                    let n = slots(&self.p.types, ty);
                    let part = vals.get(at..at + n).unwrap_or(&[]).to_vec();
                    self.bind(p, &part, ty)?;
                }
            }
        }
        Ok(())
    }
}
