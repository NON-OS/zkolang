/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `for` over a constant range, and over an array's elements with their indices. */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::ssa::V;
use crate::compiler::tir::{LocalId, TBlock, TExpr, TLit, TPat};

/** The most iterations one loop may unroll to. */
const MAX_ITERATIONS: i128 = 1 << 22;

impl<'p> Lower<'p> {
    /** `for var in lo..hi`, or `..=hi` when inclusive, the bounds constant. */
    pub(super) fn for_range(
        &mut self,
        var: LocalId,
        range: (&TExpr, &TExpr, bool),
        body: &TBlock,
    ) -> L<()> {
        let (lo, hi, inclusive) = range;
        let ty = lo.ty;
        let (lo, hi) = (
            self.constant(lo)?,
            self.constant(hi)? + i128::from(inclusive),
        );
        if hi - lo > MAX_ITERATIONS {
            return Err(LowerError::TooLarge);
        }
        for k in lo..hi.max(lo) {
            if self.dead() {
                break;
            }
            let v = self.lit(TLit::Int(k), ty);
            self.set_local(var, v);
            self.iteration(body)?;
        }
        Ok(())
    }

    /** `for pat in array`, or `for (index, pat) in array.enumerate()`. */
    pub(super) fn for_array(
        &mut self,
        index: Option<LocalId>,
        pat: &TPat,
        array: &TExpr,
        body: &TBlock,
    ) -> L<()> {
        let items = self.expr(array)?;
        let Some((el, size, n)) = super::layout::elements(&self.p.types, array.ty) else {
            return Err(LowerError::Unsupported(
                "a loop over this value",
                array.span,
            ));
        };
        for k in 0..n {
            if self.dead() {
                break;
            }
            if let Some(i) = index {
                let kv = self.b.konst(k as i128);
                self.set_local(i, alloc::vec![kv]);
            }
            let item: Vec<V> = items.get(k * size..(k + 1) * size).unwrap_or(&[]).to_vec();
            self.bind(pat, &item, el)?;
            self.iteration(body)?;
        }
        Ok(())
    }
}
