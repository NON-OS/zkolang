// NONOS Operating System (AGPL-3.0-or-later)
//! The consistency-index bindings, one block per inner query: query q's region-6
//! derived point is the x every DEEP term of query q divides by, and its bits are
//! the authenticated path directions of every one of q's consistency openings, so
//! all of q's openings open the index q's point is derived from. Level zero lives
//! in the opened-cell column choice. This is the forgery-critical seam: q's DEEP
//! divides by q's own opened index, never another query's.

use super::super::layout::Layout;
use super::helpers::{chain, group};
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

pub(crate) fn index(lay: &Layout, out: &mut Vec<GpGroup>) {
    let l = lay.l;
    for q in 0..lay.n_q {
        let i_off = lay.i_off[q];
        let d_off = lay.d_off[q];
        let m_off = lay.m_off[q];

        out.push(group(
            lay.span,
            alloc::vec![1, 2, 14, 15],
            &[(i_off + lay.pbits, 1, d_off, 14), (i_off + lay.pbits, 2, d_off, 15)],
        ));

        let span5 = lay.ocells[q][1].0;
        let mut sw: Vec<(usize, usize, usize, usize)> = Vec::new();
        for m in 1..lay.depth {
            let mut cells: Vec<(usize, usize)> = alloc::vec![(i_off + m, 0)];
            for o in 1..lay.n_open {
                cells.push((m_off + o * span5 + m * l - 1, 8));
            }
            chain(&cells, &mut sw);
        }
        out.push(group(lay.span, alloc::vec![0, 8], &sw));
    }
}
