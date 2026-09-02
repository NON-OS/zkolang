// NONOS Operating System (AGPL-3.0-or-later)
//! The periodic bindings. Plain path: each recomputed P_j(z), on its run's
//! last row, is the compose input cell that consumes it. Sidecar path: the
//! recompute region does not exist; the claim absorbed in the transcript is
//! the compose input and every query's deep quotient claim, one three-way tie
//! per column, and the opened rows those quotients divide are bound to the
//! authenticated chunks elsewhere.

use super::super::layout::Layout;
use super::helpers::Bind;
use super::helpers::{chain, group, labeled};
use alloc::vec::Vec;

pub fn periodic(lay: &Layout, out: &mut Vec<Bind>) {
    if lay.sidecar {
        let l = lay.l;
        let base = lay.width_inner * lay.window_inner + 1;
        for j in 0..lay.n_pz {
            let (pc0, pc1) = (lay.c_periodic_col + 2 * j, lay.c_periodic_col + 2 * j + 1);
            for lane in 0..2 {
                let claim_row = (lay.claim_op + 2 * j + lane) * l;
                let cc = if lane == 0 { pc0 } else { pc1 };
                let mut cells: Vec<(usize, usize)> = alloc::vec![(claim_row, 8), (lay.c_off, cc)];
                for q in 0..lay.n_q {
                    cells.push((lay.d_off[q] + base + j, 8 + lane));
                }
                let mut sw = Vec::new();
                chain(&cells, &mut sw);
                let mut wcols: Vec<usize> = cells.iter().map(|c| c.1).collect();
                wcols.sort_unstable();
                wcols.dedup();
                out.push(labeled("claim", lay.span, wcols, &sw));
            }
        }
        return;
    }
    for j in 0..lay.n_pz {
        let r = lay.pz_off + (j + 1) * lay.t_inner - 1;
        let (pc0, pc1) = (lay.c_periodic_col + 2 * j, lay.c_periodic_col + 2 * j + 1);
        out.push(group(
            lay.span,
            alloc::vec![10, 11, pc0, pc1],
            &[(r, 10, lay.c_off, pc0), (r, 11, lay.c_off, pc1)],
        ));
    }
}
