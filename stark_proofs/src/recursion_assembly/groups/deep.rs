// NONOS Operating System (AGPL-3.0-or-later)
//! The DEEP bindings, one block per inner query: the check's z and coefficients
//! to the (shared) transcript, its batched result and composition value to query
//! q's authenticated openings, every trace value to query q's opening, and the
//! claims cycled through the (shared) composition frame and transcript-absorbed
//! frame. Every cell in query q's block uses q's own DEEP and auth offsets — no
//! cross-query reference, so q's algebra divides by q's opened index.

use super::super::layout::Layout;
use super::helpers::{cycle, group};
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

pub(crate) fn deep(lay: &Layout, out: &mut Vec<GpGroup>) {
    let l = lay.l;
    for q in 0..lay.n_q {
        let d_off = lay.d_off[q];
        let m_off = lay.m_off[q];

        // The DEEP z == the transcript's squeezed out-of-domain point (shared).
        out.push(group(
            lay.span,
            alloc::vec![0, 10, 11],
            &[(lay.z_op * l, 0, d_off + 1, 10), ((lay.z_op + 1) * l, 0, d_off + 1, 11)],
        ));
        // The batched result == query q's authenticated DEEP opening.
        let (dr, dc) = (m_off + lay.ocells[q][1].0, lay.ocells[q][1].1);
        out.push(group(
            lay.span,
            alloc::vec![2, 3, dc, dc + 1],
            &[(d_off + lay.n_terms, 2, dr, dc), (d_off + lay.n_terms, 3, dr, dc + 1)],
        ));
        // The composition term's value == query q's authenticated composition opening.
        let (cr, cc) = (m_off + lay.ocells[q][2].0, lay.ocells[q][2].1);
        out.push(group(
            lay.span,
            alloc::vec![6, 7, cc, cc + 1],
            &[
                (d_off + lay.n_terms - 1, 6, cr, cc),
                (d_off + lay.n_terms - 1, 7, cr, cc + 1),
            ],
        ));

        // Every trace value feeding query q's batch == q's authenticated opening.
        for c in 0..lay.width_inner {
            let leaf_row = m_off + lay.ocells[q][3 + c].0;
            let leaf_col = lay.ocells[q][3 + c].1;
            for lane in 0..2 {
                let mut cells: Vec<(usize, usize)> = (0..lay.window_inner)
                    .map(|k| (d_off + k * lay.width_inner + c, 6 + lane))
                    .collect();
                cells.push((leaf_row, leaf_col + lane));
                out.push(cycle(lay.span, &cells));
            }
        }

        // Each batching coefficient == its transcript squeeze (shared).
        for i in 0..lay.n_terms {
            let op = lay.deep_coeff_op + 2 * i;
            out.push(group(
                lay.span,
                alloc::vec![0, 12, 13],
                &[(op * l, 0, d_off + i, 12), ((op + 1) * l, 0, d_off + i, 13)],
            ));
        }

        // The claims == the (shared) composition frame == the transcript-absorbed
        // frame. Every query's DEEP claim binds to the one shared frame.
        for i in 0..lay.frame_len {
            let ood_c0 = lay.z_op + 2 + 2 * i;
            out.push(cycle(lay.span, &[(lay.c_off, 2 * i), (d_off + i, 8), (ood_c0 * l, 8)]));
            out.push(cycle(
                lay.span,
                &[(lay.c_off, 2 * i + 1), (d_off + i, 9), ((ood_c0 + 1) * l, 8)],
            ));
        }
    }
}
