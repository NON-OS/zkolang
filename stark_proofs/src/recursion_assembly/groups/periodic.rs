// NONOS Operating System (AGPL-3.0-or-later)
//! The periodic bindings: each recomputed P_j(z), on its run's last row, is
//! the compose input cell that consumes it, so the composition runs on the
//! derived evaluations rather than prover inputs.

use super::super::layout::Layout;
use super::helpers::group;
use super::helpers::Bind;
use alloc::vec::Vec;

pub fn periodic(lay: &Layout, out: &mut Vec<Bind>) {
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
