// NONOS Operating System (AGPL-3.0-or-later)
// The same question as free_wired_cells, asked of the circuit that guards the
// pool rather than of the outer.
//
// The join-split argues its copy constraint at beta = 5 and gamma = 7, set in
// four places under shield/wire*.rs, never drawn. So the forgery
// wired_forgery_tests builds against the outer applies here too, if the
// circuit leaves cells for it to spend: a cell no region constrains, in a class
// with a partner, in a group that holds a second such cell.
//
// This matters more than the outer finding. A forged outer fakes a settlement.
// A forged inner fakes a spend, which is the statement the pool pays out on.

use crate::crypto::stark::air::Air;
use crate::crypto::stark::field::Fp;
use crate::shield::key::Break;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

/// The join-split leaves nothing a compensated forgery can spend.
///
/// Not ignored, and not a report. The circuit is argued at constants a prover
/// reads off the layout, and the only reason that is survivable is that every
/// cell its permutation binds is also held by a region's own rules. That is a
/// measured property of the wiring today, not a design invariant, and one new
/// region carrying witness cells would end it silently.
///
/// So it is a gate. If this ever fails, the pool's spend statement has become
/// forgeable and the two round Poseidon prover is no longer optional.
#[test]
fn the_join_split_leaves_no_cell_a_forgery_could_spend() {
    let (per_group, total_bound) = free_bound_cells();
    assert!(
        !per_group.iter().any(|&n| n >= 2),
        "a group holds two cells nothing but the wiring binds, out of {total_bound} bound: \
         the spend statement is now forgeable at its fixed challenges, per group {per_group:?}"
    );
}

/// Per group, how many of its bound cells no region constrains, and how many
/// cells the permutation binds at all.
fn free_bound_cells() -> (Vec<usize>, usize) {
    let js = crate::shield::test::scenario::balanced_deployed(Break::None);
    let stride = Air::trace_width(&js.wired);
    let total = js.witness.len() / stride;
    let periodic = Air::periodic_columns(&js.wired);
    let groups = js.wired.group_params();
    let n_groups = groups.len();
    let width = stride - n_groups;

    let at = |trace: &[Fp], r: usize| -> Vec<Fp> {
        let w = &trace[r * stride..(r + 2) * stride];
        let p: Vec<Fp> = periodic.iter().map(|col| col[r]).collect();
        Air::transition(&js.wired, w, &p)
    };
    let regions_quiet = |v: &[Fp]| v[..v.len() - n_groups].iter().all(|x| *x == Fp::ZERO);
    let pinned: BTreeSet<usize> = Air::boundary(&js.wired)
        .into_iter()
        .filter(|&(col, _, _)| col < width)
        .map(|(col, row, _)| row * width + col)
        .collect();

    let sigmas = js.wired.group_sigmas();
    let mut bound: Vec<(usize, usize, usize)> = Vec::new();
    for (gi, (cols, _, _)) in groups.iter().enumerate() {
        let k = cols.len();
        let sigma = sigmas[gi];
        for r in 0..total {
            for (j, &c) in cols.iter().enumerate() {
                let slot = r * k + j;
                if slot < sigma.len() && sigma[slot] != slot {
                    bound.push((r, c, gi));
                }
            }
        }
    }

    let mut trace = js.witness.clone();
    let mut per_group = alloc::vec![0usize; n_groups];
    for &(r, c, gi) in &bound {
        if r + 1 >= total || pinned.contains(&(r * width + c)) {
            continue;
        }
        let idx = r * stride + c;
        let saved = trace[idx];
        trace[idx] = saved + Fp::ONE;
        let quiet = regions_quiet(&at(&trace, r)) && (r == 0 || regions_quiet(&at(&trace, r - 1)));
        trace[idx] = saved;
        if quiet {
            per_group[gi] += 1;
        }
    }
    (per_group, bound.len())
}

#[test]
#[ignore]
fn how_many_cells_the_join_split_leaves_spendable() {
    let js = crate::shield::test::scenario::balanced_deployed(Break::None);
    let stride = Air::trace_width(&js.wired);
    let total = js.witness.len() / stride;
    let periodic = Air::periodic_columns(&js.wired);
    let groups = js.wired.group_params();
    let n_groups = groups.len();
    let width = stride - n_groups;

    std::println!(
        "join-split trace {total} rows x {stride} cols, {n_groups} groups over {} wired columns",
        groups
            .iter()
            .flat_map(|(cols, _, _)| cols.iter().copied())
            .collect::<BTreeSet<usize>>()
            .len()
    );

    let at = |trace: &[Fp], r: usize| -> Vec<Fp> {
        let w = &trace[r * stride..(r + 2) * stride];
        let p: Vec<Fp> = periodic.iter().map(|col| col[r]).collect();
        Air::transition(&js.wired, w, &p)
    };
    let regions_quiet = |v: &[Fp]| v[..v.len() - n_groups].iter().all(|x| *x == Fp::ZERO);

    for r in 0..total - 1 {
        assert!(
            at(&js.witness, r).iter().all(|x| *x == Fp::ZERO),
            "the honest join-split witness fails at row {r}, so this probe is measuring itself"
        );
    }

    let pinned: BTreeSet<usize> = Air::boundary(&js.wired)
        .into_iter()
        .filter(|&(col, _, _)| col < width)
        .map(|(col, row, _)| row * width + col)
        .collect();

    /*
     * Only cells the permutation actually moves. A slot sigma fixes is in no
     * class: its two factors are the same value and cancel, so moving that
     * cell neither breaks a binding nor shifts the product, and it is useless
     * to a forgery as victim or as payer. Counting those would have said this
     * circuit leaks 68,169 cells when most of them are simply unwired.
     */
    let sigmas = js.wired.group_sigmas();
    let mut bound: Vec<(usize, usize, usize)> = Vec::new();
    for (gi, (cols, _, _)) in groups.iter().enumerate() {
        let k = cols.len();
        let sigma = sigmas[gi];
        for r in 0..total {
            for (j, &c) in cols.iter().enumerate() {
                let slot = r * k + j;
                if slot < sigma.len() && sigma[slot] != slot {
                    bound.push((r, c, gi));
                }
            }
        }
    }
    std::println!("cells the permutation moves: {}", bound.len());

    let mut trace = js.witness.clone();
    let mut per_group = alloc::vec![0usize; n_groups];
    let mut moved: Vec<(usize, usize)> = Vec::new();
    for &(r, c, gi) in &bound {
        if r + 1 >= total || pinned.contains(&(r * width + c)) {
            continue;
        }
        let idx = r * stride + c;
        let saved = trace[idx];
        trace[idx] = saved + Fp::ONE;
        let quiet = regions_quiet(&at(&trace, r)) && (r == 0 || regions_quiet(&at(&trace, r - 1)));
        trace[idx] = saved;
        if quiet {
            per_group[gi] += 1;
            if moved.len() < 8 {
                moved.push((r, c));
            }
        }
    }

    let free: usize = per_group.iter().sum();
    std::println!("of those, held only by the wiring: {free}, per group {per_group:?}");
    std::println!("first few at {moved:?}");
    /*
     * A forgery needs two in one group: one to break and one to pay with, and
     * one unknown only solves one equation.
     */
    std::println!(
        "{}",
        if per_group.iter().any(|&n| n >= 2) {
            "one group holds two cells nothing but the wiring binds, so the spend \
             statement is forgeable at its fixed challenges"
        } else {
            "no group holds two such cells, so the two cell construction does not \
             close on this circuit; the argument is still made at a point the \
             prover knows"
        }
    );
}
