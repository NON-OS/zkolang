// NONOS Operating System (AGPL-3.0-or-later)
//! The strip side of the compose split: record the inner's transition, emit
//! the plan, place the witness from the real out-of-domain values, and name
//! every cycle the assembly must bind: each echo cell to its producer (a
//! strip lane or a flat statement cell) and each final accumulator to its
//! acc cell in the flat region.

use super::inner::Inner;
use crate::compose_pipeline::{eval, snapshot, strip_layout, strip_plan, Cell, Node};
use crate::crypto::stark::air::{AirExt, ComposeStrip, GenericTransition, OutStatement, EMPTY};
use crate::crypto::stark::field::{Ext2, Fp};
use alloc::vec::Vec;

/// A cycle endpoint outside the strip: a base column in the flat region.
pub enum Home {
    Flat(usize),
    Strip(usize, usize),
}

pub struct StripSide {
    pub region: ComposeStrip,
    pub trace: Vec<Fp>,
    pub stmt: Vec<OutStatement>,
    /// The shape the layout emit carries for a config-driven verifier.
    pub k: usize,
    pub echo_width: usize,
    pub n_out: usize,
    pub used_rows: usize,
    /// Every echo cell as ((strip row, strip col), producer home).
    pub cycles: Vec<((usize, usize), Home)>,
    /// Per base output j: the strip's acc column, read at the final row.
    pub acc_cols: Vec<usize>,
    pub final_row: usize,
}

pub fn strip_side<A: AirExt + GenericTransition>(inner: &Inner<A>) -> StripSide {
    let w = inner.proof.ood_frame.len();
    let p = inner.ci.periodic_z.len();
    let cells = crate::compose_pipeline::begin(2 * (w + p));
    let frame: Vec<Ext2<Cell>> =
        (0..w).map(|i| Ext2::new(cells[2 * i], cells[2 * i + 1])).collect();
    let per: Vec<Ext2<Cell>> = (0..p)
        .map(|i| Ext2::new(cells[2 * (w + i)], cells[2 * (w + i) + 1]))
        .collect();
    let outs = inner.air.transition_gen::<Ext2<Cell>>(&frame, &per);
    let tape = snapshot();
    let out_ids: Vec<u32> = outs.iter().flat_map(|o| [o.c0.0, o.c1.0]).collect();

    let lay = strip_layout(&tape, &out_ids, 4);
    let (plan, stmt) = strip_plan(&tape, &lay);

    let mut inputs = Vec::with_capacity(2 * (w + p));
    for v in &inner.proof.ood_frame {
        inputs.push(v.c0);
        inputs.push(v.c1);
    }
    for v in &inner.ci.periodic_z {
        inputs.push(v.c0);
        inputs.push(v.c1);
    }
    let vals = eval(&tape, &inputs);

    // Lane placement of every witnessed product, for the echo homes.
    let mut place: alloc::collections::BTreeMap<u32, (usize, usize)> = Default::default();
    for (r, row) in plan.rows.iter().enumerate() {
        for (l, id) in row.lanes.iter().enumerate() {
            if *id != EMPTY {
                place.insert(*id, (r, l));
            }
        }
    }

    let region = ComposeStrip::new(plan.clone());
    let trace = region.trace(&vals);
    let mut cycles = Vec::new();
    for (r, row) in plan.rows.iter().enumerate() {
        let _ = row;
        for (col, id) in region.echo_cells(r) {
            let home = match &tape[id as usize] {
                Node::Input(u) => Home::Flat(*u as usize),
                _ => {
                    let (pr, pl) = place[&id];
                    Home::Strip(pr, pl)
                }
            };
            cycles.push(((r, col), home));
        }
    }

    let n_out = plan.n_out;
    let final_row = (1usize << crate::crypto::stark::air::Air::log_trace_len(&region)) - 1;
    StripSide {
        acc_cols: (0..n_out).map(|j| region.acc_col(j)).collect(),
        k: plan.k,
        echo_width: plan.echo_width,
        n_out,
        used_rows: plan.rows.len(),
        region,
        trace,
        stmt,
        cycles,
        final_row,
    }
}
