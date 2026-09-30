/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The cost report (section 15.2): each row of a built program's machine code attributed
 * to the site its instruction comes from, and so to the function whose code it is and the
 * functions inlined to reach it; with the peak number of live registers.
 */

use alloc::collections::BTreeMap;

use super::built::Built;
use super::cost_rows::{live_counts, owners};
use crate::compiler::source::Span;
use crate::compiler::tir::FnId;

/** The rows of one function, and the most registers live at a row of its own. */
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FnCost {
    /** Its rows and those of the functions inlined into it. */
    pub inclusive: usize,
    /** The rows of its own code. */
    pub exclusive: usize,
    pub peak: usize,
}

/** Where a built program's rows come from. */
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cost {
    pub rows: usize,
    pub fns: BTreeMap<FnId, FnCost>,
    /** The rows of each expression's own instructions, by its span. */
    pub spans: BTreeMap<Span, usize>,
    /** Rows of no function: reading and checking the inputs, padding and the final `Halt`. */
    pub overhead: usize,
    pub peak: usize,
}

/** The cost report of `b`. */
pub fn cost(b: &Built) -> Cost {
    let m = &b.compiled.machine;
    let live = live_counts(&m.ops);
    let mut c = Cost {
        rows: m.ops.len(),
        peak: live.iter().copied().max().unwrap_or(0),
        ..Cost::default()
    };
    let whose = owners(&m.origins);
    for (i, v) in whose.iter().enumerate() {
        let Some(v) = v else {
            c.overhead = c.overhead.saturating_add(1);
            continue;
        };
        let place = b.sites.place(b.compiled.ssa.site(v.index()));
        if let Some(p) = place {
            let n = c.spans.entry(p.span).or_default();
            *n = n.saturating_add(1);
        }
        let chain = place.map_or(&[][..], |p| &p.chain[..]);
        c.charge(chain, live.get(i).copied().unwrap_or(0));
    }
    c
}
