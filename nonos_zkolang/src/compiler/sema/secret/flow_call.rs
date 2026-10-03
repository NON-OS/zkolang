/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Calls. The callee's summary gives, from the labels of the arguments' parts, the labels
 * of the result and of each `&mut` argument afterwards, and names the parts that must be
 * public; an argument has its parameter's type, so its parts come in the order of the
 * parameter's slots. A callee with no summary, one that failed to check, is taken to mix every
 * argument into its result and its `&mut` arguments.
 */

use alloc::vec::Vec;

use super::flow::Flow;
use super::shape::Shape;
use super::slots::flatten;
use super::taint::Taint;
use crate::compiler::tir::{FnId, TArg, TExpr, TPlace};

impl<'p> Flow<'p> {
    /** The labels of the value of the call `e` of `f` on `args`. */
    pub(super) fn call(&mut self, f: FnId, args: &[TArg], e: &TExpr) -> Shape {
        let program = self.program;
        let callee = program.fns.get(f.0 as usize);
        let mut flat = Vec::new();
        let mut ranges = Vec::with_capacity(args.len());
        let mut places: Vec<(usize, &TPlace, Taint)> = Vec::new();
        for (i, a) in args.iter().enumerate() {
            let (v, ty) = match a {
                TArg::Value(v) => (self.expr(v), v.ty),
                TArg::Place(p) => {
                    let index = self.index_taint(p);
                    places.push((i, p, index));
                    (self.read(p).raised(index), p.ty)
                }
            };
            let start = flat.len();
            flatten(&v, ty, &program.types, &mut flat);
            ranges.push(start..flat.len());
        }
        let Some(Some(s)) = self.summaries.get(f.0 as usize) else {
            let t = flat.iter().fold(Taint::PUBLIC, |a, b| a.join(*b));
            for (_, p, index) in places {
                self.write(p, Shape::Leaf(t.join(self.pc)), index, p.span);
            }
            return Shape::of(e.ty, t, &program.types);
        };
        let arg = |k: usize| flat.get(k).copied().unwrap_or(Taint::SECRET);
        let name = callee.map_or("", |c| c.name.as_str());
        let what = alloc::format!("a parameter of `{name}` that must be public");
        for (a, range) in args.iter().zip(&ranges) {
            let needed = range
                .clone()
                .filter(|&k| k < 128 && s.needs_public & (1u128 << k) != 0);
            let t = needed.fold(Taint::PUBLIC, |t, k| t.join(arg(k)));
            let at = match a {
                TArg::Value(v) => v.span,
                TArg::Place(p) => p.span,
            };
            self.require_public(t, at, &what);
        }
        for (i, p, index) in places {
            if let Some(Some(out)) = s.ref_out.get(i) {
                self.write(p, out.apply(&arg).raised(self.pc), index, p.span);
            }
        }
        s.result.apply(&arg)
    }
}
