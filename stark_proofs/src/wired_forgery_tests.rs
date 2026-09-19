// NONOS Operating System (AGPL-3.0-or-later)
// The fixed challenge forgery, carried out on the real outer rather than on a
// toy.
//
// `wired_challenge_tests` shows the mechanism works and `free_wired_cells`
// shows this circuit has 1,324 cells nothing but the wiring holds, spread over
// 350 classes. Neither of those is the same as doing it. This does it: pick a
// wired cell the regions do not constrain, move it so a copy constraint no
// longer holds, solve one linear equation for a second free cell that puts the
// group's product back, and ask the assembly whether it is satisfied.
//
// If it is, the outer accepts a witness in which two cells the circuit says
// are equal are not, and it accepts it because the point the product is
// checked at was a constant the forger could read off the layout.
//
// Ignored by default: it assembles the real outer, which is minutes and gigabytes.

use crate::crypto::stark::air::{Air, GpGroup};
use crate::crypto::stark::field::Fp;
use crate::recursion_assembly::build::{assemble_real_capped, binds_for};
use crate::recursion_assembly::groups;
use crate::recursion_assembly::Tamper;
use crate::witness_satisfies::satisfies;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

/// Every cell the wiring touches, ascending.
fn wired_cells(classes: &[Vec<usize>]) -> Vec<usize> {
    let mut v: Vec<usize> = classes.iter().flat_map(|c| c.iter().copied()).collect();
    v.sort_unstable();
    v.dedup();
    v
}

/// A cell's factor in one group, as (numerator, denominator).
fn factor(g: &GpGroup, k: usize, row: usize, slot: usize, v: Fp) -> (Fp, Fp) {
    let id = row * k + slot;
    (
        v + g.beta * Fp::from_u64(id as u64) + g.gamma,
        v + g.beta * Fp::from_u64(g.sigma[id] as u64) + g.gamma,
    )
}

#[test]
#[ignore]
fn the_real_outer_accepts_a_compensated_forgery() {
    let asm = assemble_real_capped(Tamper::None, 1);
    let stride = Air::trace_width(&asm.wired);
    let width = stride - asm.n_groups;
    let total = asm.witness.len() / stride;
    let span = asm.lay.span;
    let periodic = Air::periodic_columns(&asm.wired);

    assert!(
        satisfies(&asm.wired, &asm.witness),
        "the honest witness does not satisfy the assembly, so nothing below means anything"
    );

    let binds = binds_for(&asm.lay);
    let classes = groups::wiring_classes(&binds, span, width);
    let gps = groups::collapse(&binds, span, width);
    assert_eq!(
        gps.len(),
        asm.n_groups,
        "the groups rebuilt here are not the ones the assembly used"
    );

    /*
     * Which groups a column sits in. A cell moved in a column that several
     * groups hold breaks all of their products at once, and one unknown
     * cannot pay for more than one equation, so the forgery wants columns held
     * by exactly one group, and the same one for both cells.
     */
    let mut of_column: Vec<Vec<usize>> = alloc::vec![Vec::new(); width];
    for (gi, g) in gps.iter().enumerate() {
        for &c in &g.wired_cols {
            of_column[c].push(gi);
        }
    }

    /*
     * Free cells, by the same question `free_wired_cells` asks: move it and
     * see whether any region constraint or boundary objects. Products are
     * ignored here because no region reads them and the forger refills them.
     */
    let at = |trace: &[Fp], r: usize| -> Vec<Fp> {
        let w = &trace[r * stride..(r + 2) * stride];
        let p: Vec<Fp> = periodic.iter().map(|col| col[r]).collect();
        Air::transition(&asm.wired, w, &p)
    };
    let regions_quiet = |v: &[Fp]| v[..v.len() - asm.n_groups].iter().all(|x| *x == Fp::ZERO);
    let pinned: BTreeSet<usize> = Air::boundary(&asm.wired)
        .into_iter()
        .filter(|&(col, _, _)| col < width)
        .map(|(col, row, _)| row * width + col)
        .collect();

    let mut trace = asm.witness.clone();
    let mut is_free = |trace: &mut Vec<Fp>, cell: usize| -> bool {
        let (r, c) = (cell / width, cell % width);
        if r + 1 >= total || pinned.contains(&cell) {
            return false;
        }
        let idx = r * stride + c;
        let saved = trace[idx];
        trace[idx] = saved + Fp::ONE;
        let quiet = regions_quiet(&at(trace, r)) && (r == 0 || regions_quiet(&at(trace, r - 1)));
        trace[idx] = saved;
        quiet
    };

    /*
     * Which class a cell belongs to, so a candidate can be told whether moving
     * it breaks anything. A class of one is a fixed point and breaks nothing.
     */
    let mut class_of: Vec<usize> = alloc::vec![usize::MAX; span * width];
    for (ci, class) in classes.iter().enumerate() {
        for &cell in class {
            class_of[cell] = ci;
        }
    }

    /*
     * Free cells whose column belongs to exactly one group. One unknown pays
     * for one equation, so a cell held by several groups breaks more products
     * than a single payer can put back.
     */
    let mut free_in_group: Vec<Vec<usize>> = alloc::vec![Vec::new(); gps.len()];
    let mut n_free = 0usize;
    for &cell in wired_cells(&classes).iter() {
        let c = cell % width;
        if of_column[c].len() != 1 || !is_free(&mut trace, cell) {
            continue;
        }
        n_free += 1;
        free_in_group[of_column[c][0]].push(cell);
    }

    /*
     * Any group holding two of them, the first in a class with a partner to
     * break away from.
     */
    let slot_of = |g: &GpGroup, c: usize| g.wired_cols.iter().position(|&x| x == c).unwrap();
    let mut chosen: Option<(usize, usize, usize, usize)> = None;
    for (gi, cells) in free_in_group.iter().enumerate() {
        if cells.len() < 2 {
            continue;
        }
        for &v in cells {
            let ci = class_of[v];
            if ci == usize::MAX || classes[ci].len() < 2 {
                continue;
            }
            if let Some(&p) = cells.iter().find(|&&p| p != v) {
                let partner = classes[ci].iter().copied().find(|&o| o != v).unwrap();
                chosen = Some((v, partner, p, gi));
                break;
            }
        }
        if chosen.is_some() {
            break;
        }
    }

    let sizes: Vec<usize> = free_in_group.iter().map(|c| c.len()).filter(|&n| n > 0).collect();
    let (vcell, vpartner, pcell, gi) = chosen.unwrap_or_else(|| {
        panic!(
            "no group holds two free cells with one of them in a class: \
             {n_free} free cells in singly held columns, spread over {} groups, \
             largest holding {}",
            sizes.len(),
            sizes.iter().max().copied().unwrap_or(0)
        )
    });
    let g = &gps[gi];
    let k = g.wired_cols.len();

    let (vr, vc) = (vcell / width, vcell % width);
    let (pr, pc) = (pcell / width, pcell % width);
    let (vslot, pslot) = (slot_of(g, vc), slot_of(g, pc));
    let (vidx, pidx) = (vr * stride + vc, pr * stride + pc);
    let (v0, p0) = (trace[vidx], trace[pidx]);

    /*
     * Break it, then solve. Writing the moved cell as a and the paying cell as
     * x, the group's product is unchanged when
     *
     *     fa(a) * fx(x) * ga(a0) * gx(x0) == ga(a) * gx(x) * fa(a0) * fx(x0)
     *
     * and both fx and gx are linear in x, so x falls out in one step.
     */
    let a = v0 + Fp::ONE;
    let (fa, ga) = factor(g, k, vr, vslot, a);
    let (fa0, ga0) = factor(g, k, vr, vslot, v0);
    let (fx0, gx0) = factor(g, k, pr, pslot, p0);
    let lhs = fa * ga0 * gx0;
    let rhs = ga * fa0 * fx0;
    // lhs * (x + beta*id + gamma) == rhs * (x + beta*sigma + gamma)
    let pid = pr * k + pslot;
    let shift_f = g.beta * Fp::from_u64(pid as u64) + g.gamma;
    let shift_g = g.beta * Fp::from_u64(g.sigma[pid] as u64) + g.gamma;
    let x = (rhs * shift_g - lhs * shift_f) * (lhs - rhs).inv();

    trace[vidx] = a;
    trace[pidx] = x;
    asm.wired.refill_products(&mut trace);

    let partner_value = trace[(vpartner / width) * stride + (vpartner % width)];
    assert_ne!(
        trace[vidx], partner_value,
        "the forged witness still holds the copy constraint, so it forges nothing"
    );
    assert!(
        satisfies(&asm.wired, &trace),
        "the real outer refused the compensated forgery, so either a region holds \
         one of these cells after all or the compensation is wrong"
    );
}
