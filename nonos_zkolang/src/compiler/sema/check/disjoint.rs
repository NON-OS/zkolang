/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Two places passed to one call must not overlap (section 10.3), which is an error unless
 * the checker can see they are disjoint: different tuple fields, or elements at different
 * literal indices.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::tir::{Proj, TArg, TExprKind, TLit, TPlace};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report two `&mut` arguments of one call that may overlap (E0310). */
    pub(crate) fn check_disjoint(&mut self, args: &[TArg]) {
        let places: Vec<&TPlace> = args
            .iter()
            .filter_map(|a| match a {
                TArg::Place(p) => Some(p),
                TArg::Value(_) => None,
            })
            .collect();
        for (i, a) in places.iter().enumerate() {
            for b in places.iter().skip(i + 1) {
                if a.root == b.root && !disjoint(&a.proj, &b.proj) {
                    let d = Diagnostic::error(Code::ALIASED_MUT, "two `&mut` arguments may name the same place", b.span, "may overlap")
                        .with_label(a.span, "the other")
                        .with_help("a call sees each `&mut` place once; pass disjoint parts, or copy one first");
                    self.sema.diags.push(d);
                }
            }
        }
    }
}

/** Whether two paths into one local are sure to name disjoint parts. */
fn disjoint(a: &[Proj], b: &[Proj]) -> bool {
    a.iter().zip(b).any(|(x, y)| match (x, y) {
        (Proj::TupleField(i), Proj::TupleField(j)) => i != j,
        (Proj::Index(i), Proj::Index(j)) => match (&i.kind, &j.kind) {
            (TExprKind::Lit(TLit::Int(m)), TExprKind::Lit(TLit::Int(n))) => m != n,
            _ => false,
        },
        _ => false,
    })
}
