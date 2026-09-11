// NONOS Operating System (AGPL-3.0-or-later)
//! From layout to the region's plan. The emitter produces the `StripPlan`
//! the `ComposeStrip` region consumes directly, one source of truth for the
//! shape: an inert init row first, dense operand schedules per lane, and the
//! output accumulators over products only. Statement contributions (inputs
//! and folded constants) do not ride the strip; they come back separately as
//! the flat region's pin data, evaluated there over its own statement cells.

use super::layout::{Source, StripLayout};
use super::replay::eval;
use super::tape::Node;
use crate::crypto::stark::air::{ComposeStrip, OpSched, OutStatement, RowSched, StripPlan, EMPTY};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// Compile the layout into the region's plan and the flat-pin data.
pub fn strip_plan(tape: &[Node], lay: &StripLayout) -> (StripPlan, Vec<OutStatement>) {
    let k = lay.k;
    let echo_width = lay.rows.iter().map(|r| r.echoes.len()).max().unwrap_or(0);
    let n_cand = 2 * k + echo_width;
    let n_out = lay.out_terms.len();

    let zero_op = || OpSched { coeffs: alloc::vec![Fp::ZERO; n_cand], constant: Fp::ZERO };
    let dense = |form: &super::layout::LinForm| -> OpSched {
        let mut o = zero_op();
        for (c, s) in &form.terms {
            let idx = match s {
                Source::Slot { rel: 0, lane } => *lane as usize,
                Source::Slot { rel: _, lane } => k + *lane as usize,
                Source::Echo { idx } => 2 * k + *idx as usize,
            };
            o.coeffs[idx] = o.coeffs[idx] + *c;
        }
        o.constant = form.constant;
        o
    };

    let init = RowSched {
        lanes: alloc::vec![EMPTY; k],
        echoes: Vec::new(),
        ops: (0..k).map(|_| (zero_op(), zero_op())).collect(),
        acc: alloc::vec![alloc::vec![Fp::ZERO; k]; n_out],
    };
    let mut rows = alloc::vec![init];
    for lrow in &lay.rows {
        let mut lanes = alloc::vec![EMPTY; k];
        let mut ops: Vec<(OpSched, OpSched)> = Vec::with_capacity(k);
        for (l, lane) in lrow.lanes.iter().enumerate() {
            lanes[l] = lane.node;
            ops.push((dense(&lane.a), dense(&lane.b)));
        }
        while ops.len() < k {
            ops.push((zero_op(), zero_op()));
        }
        rows.push(RowSched {
            lanes,
            echoes: lrow.echoes.clone(),
            ops,
            acc: alloc::vec![alloc::vec![Fp::ZERO; k]; n_out],
        });
    }

    let mut stmt: Vec<OutStatement> = lay
        .out_consts
        .iter()
        .map(|c| OutStatement { input_coeffs: Vec::new(), constant: *c })
        .collect();
    for (j, terms) in lay.out_terms.iter().enumerate() {
        for (r, c, s) in terms {
            match s {
                Source::Slot { rel: _, lane } => {
                    // Shifted one row down by the init row.
                    rows[*r + 1].acc[j][*lane as usize] =
                        rows[*r + 1].acc[j][*lane as usize] + *c;
                }
                Source::Echo { idx } => {
                    let id = lay.rows[*r].echoes[*idx as usize];
                    match &tape[id as usize] {
                        Node::Input(i) => stmt[j].input_coeffs.push((*i, *c)),
                        _ => unreachable!("an output echo that is not an input"),
                    }
                }
            }
        }
    }

    (StripPlan { k, echo_width, n_out, rows }, stmt)
}

/// The region-level gate: build the region, place the witness from the
/// replayed tape, walk every window through the region's own transition with
/// its own periodic columns, check its boundary, and check that the final
/// accumulators plus the statement parts equal the tape's outputs. Returns
/// the first lying (anchor, constraint), usize::MAX row for a bad output.
pub fn check_region(
    tape: &[Node],
    plan: &StripPlan,
    stmt: &[OutStatement],
    inputs: &[Fp],
    outputs: &[u32],
) -> Option<(usize, usize)> {
    let vals = eval(tape, inputs);
    let region = ComposeStrip::new(plan.clone());
    let tr = region.trace(&vals);
    let w = crate::crypto::stark::air::Air::trace_width(&region);
    let rows = 1usize << crate::crypto::stark::air::Air::log_trace_len(&region);
    let periodic = crate::crypto::stark::air::Air::periodic_columns(&region);

    for (col, row, v) in crate::crypto::stark::air::Air::boundary(&region) {
        if tr[row * w + col] != v {
            return Some((row, col));
        }
    }
    for r in 0..rows - 1 {
        let window: Vec<Fp> = tr[r * w..(r + 2) * w].to_vec();
        let per: Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
        let res = crate::crypto::stark::air::Air::transition(&region, &window, &per);
        for (i, v) in res.iter().enumerate() {
            if *v != Fp::ZERO {
                return Some((r, i));
            }
        }
    }
    for (j, out) in outputs.iter().enumerate() {
        let mut expect = stmt[j].constant;
        for (i, c) in &stmt[j].input_coeffs {
            expect = expect + *c * inputs[*i as usize];
        }
        let acc = tr[(rows - 1) * w + region.acc_col(j)];
        if acc + expect != vals[*out as usize] {
            return Some((usize::MAX, j));
        }
    }
    None
}
