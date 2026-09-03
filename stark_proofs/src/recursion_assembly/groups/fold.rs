// NONOS Operating System (AGPL-3.0-or-later)
//! The fold bindings, one block per inner query: the betas to their (shared)
//! transcript squeezes, query q's layer-zero opened value to q's authenticated
//! FRI leaf, and the point provenance: q's layer-zero x seeded by q's region-7
//! product chain, whose bits are q's own fold position bits and q's leaf opening
//! path directions.

use super::super::layout::Layout;
use super::helpers::Bind;
use super::helpers::{chain, group};
use alloc::vec::Vec;

pub fn fold(lay: &Layout, out: &mut Vec<Bind>) {
    let l = lay.l;
    for q in 0..lay.n_q {
        let f_off = lay.f_off[q];
        let m_off = lay.m_off[q];
        let fp_off = lay.fp_off[q];

        let mut bsw: Vec<(usize, usize, usize, usize)> = Vec::new();
        for m in 0..lay.n_folds {
            bsw.push((lay.ft_off + (6 * m + 4) * l, 0, f_off + m, 0));
            bsw.push((lay.ft_off + (6 * m + 5) * l, 0, f_off + m, 1));
        }
        out.push(group(lay.span, alloc::vec![0, 1], &bsw));

        let (mr, mc) = (m_off + lay.ocells[q][0].0, lay.ocells[q][0].1);
        out.push(group(
            lay.span,
            alloc::vec![2, 3, mc, mc + 1],
            &[(f_off, 2, mr, mc), (f_off, 3, mr, mc + 1)],
        ));

        // The layer-zero point == query q's region-7 derived shift * omega^i_k, so
        // the square-and-sign chain descends from q's FRI query index.
        out.push(group(
            lay.span,
            alloc::vec![1, 6],
            &[(fp_off + lay.fbits, 1, f_off, 6)],
        ));

        // Bit k == q's fold direction at layer log_n - 2 - k == q's leaf opening's
        // path direction at level k. Level zero lives in the opened-cell column.
        let mut fsw: Vec<(usize, usize, usize, usize)> = Vec::new();
        for k in 0..lay.fbits {
            let mut cells: Vec<(usize, usize)> = alloc::vec![(fp_off + k, 0)];
            if k >= 1 {
                cells.push((m_off + k * l - 1, 8));
            }
            let fold_row = lay.log_n as usize - 2 - k;
            if fold_row < lay.n_folds {
                cells.push((f_off + fold_row, 8));
            }
            chain(&cells, &mut fsw);
        }
        out.push(group(lay.span, alloc::vec![0, 8], &fsw));
    }
}
