// NONOS Operating System (AGPL-3.0-or-later)
//! The statement bindings: the out-of-domain point cycled through the transcript
//! squeeze, the compose input, and the periodic recompute (shared); the
//! composition coefficients (shared); and comp_z into every query's DEEP check.
//! The point, coefficients, and comp_z are one shared out-of-domain object, so
//! they bind once (z, coeffs) or once per query (comp_z, to each DEEP claim).

use super::super::layout::Layout;
use super::helpers::Bind;
use super::helpers::{chain, group};
use crate::crypto::stark::air::{RATE};
use alloc::vec::Vec;

pub fn statement(lay: &Layout, out: &mut Vec<Bind>) {
    let l = lay.l;
    let (zc0, zc1) = (lay.c_z_col, lay.c_z_col + 1);
    // z: squeezed in the transcript == compose's z == the periodic recompute's z.
    let mut zsw = Vec::new();
    chain(&[(lay.z_op * l, 0), (lay.c_off, zc0), (lay.pz_off, 4)], &mut zsw);
    chain(&[((lay.z_op + 1) * l, 0), (lay.c_off, zc1), (lay.pz_off, 5)], &mut zsw);
    out.push(group(lay.span, alloc::vec![0, zc0, zc1, 4, 5], &zsw));

    // The coefficient lanes, up to two coefficients (four lanes) per group.
    let base = lay.pub_len + lay.ntr * RATE;
    let mut c = 0;
    while c < lay.n_coeff {
        let take = (lay.n_coeff - c).min(2);
        let mut wcols: Vec<usize> = alloc::vec![0];
        let mut swaps: Vec<(usize, usize, usize, usize)> = Vec::new();
        for k in 0..take {
            let bc = lay.c_coeff_col + 2 * (c + k);
            let op = base + 2 * (c + k);
            wcols.push(bc);
            wcols.push(bc + 1);
            swaps.push((op * l, 0, lay.c_off, bc));
            swaps.push(((op + 1) * l, 0, lay.c_off, bc + 1));
        }
        out.push(group(lay.span, wcols, &swaps));
        c += take;
    }

    // comp_z: the one composition value == each query's DEEP composition claim.
    let (mc0, mc1) = (lay.c_comp_z_col, lay.c_comp_z_col + 1);
    for q in 0..lay.n_q {
        out.push(group(
            lay.span,
            alloc::vec![mc0, mc1, 4, 5],
            &[(lay.c_off, mc0, lay.d_off[q], 4), (lay.c_off, mc1, lay.d_off[q], 5)],
        ));
    }
}
