// NONOS Operating System (AGPL-3.0-or-later)
//! Which wired cells of the real outer nothing but the wiring holds.
//!
//! The copy constraint is argued at fixed challenges, five and seven, which
//! the prover knows before it picks its trace. `wired_challenge_tests` builds
//! a trace that breaks a copy constraint and pays for it with a second free
//! cell, and both shapes of the argument accept it.
//!
//! What that leaves open is whether the deployed outer has the cells the
//! forgery needs. A cell some region computes is held whatever the product
//! says; only a cell nothing else constrains can be spent. Two classes of
//! those is a forgery, so the count is the answer.
//!
//! Moving a cell can only disturb the two windows it appears in, so each cell
//! costs two evaluations instead of a pass over the trace.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::crypto::stark::field::Fp;
use stark_proofs::recursion_assembly::build::{assemble_real_capped, binds_for};
use stark_proofs::recursion_assembly::groups::wiring_classes;
use stark_proofs::recursion_assembly::Tamper;
use std::collections::BTreeSet;
use std::time::Instant;

fn main() {
    let cap: usize = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(1);
    eprintln!("assembling the real outer over {cap} inner queries");
    let t = Instant::now();
    let asm = assemble_real_capped(Tamper::None, cap);
    eprintln!("assembled in {:?}", t.elapsed());

    let stride = Air::trace_width(&asm.wired);
    let width = stride - asm.n_groups;
    let total = asm.witness.len() / stride;
    let periodic = Air::periodic_columns(&asm.wired);
    let span = asm.lay.span;

    let classes = wiring_classes(&binds_for(&asm.lay), span, width);
    let wired: BTreeSet<usize> = classes.iter().flat_map(|c| c.iter().copied()).collect();
    println!(
        "trace {total} rows x {stride} cols, {} groups, {} classes over {} cells",
        asm.n_groups,
        classes.len(),
        wired.len()
    );

    /*
     * Control. If the assembly does not accept itself, the probe is measuring
     * its own mistake.
     */
    let at = |trace: &[Fp], r: usize| -> Vec<Fp> {
        let w = &trace[r * stride..(r + 2) * stride];
        let p: Vec<Fp> = periodic.iter().map(|col| col[r]).collect();
        Air::transition(&asm.wired, w, &p)
    };
    let regions_only = |v: &[Fp]| -> bool {
        // The product lanes sit after the region constraints; a forger is
        // allowed to move those, and does, so only the regions are consulted.
        let n = v.len() - asm.n_groups;
        v[..n].iter().all(|x| *x == Fp::ZERO)
    };
    for r in 0..total - 1 {
        assert!(
            at(&asm.witness, r).iter().all(|x| *x == Fp::ZERO),
            "the honest witness fails at row {r}, so this probe cannot be trusted"
        );
    }
    println!("the honest witness satisfies every constraint");

    /*
     * The products are deliberately not recomputed per cell. No region
     * constraint reads a product column, so a region's opinion of a moved cell
     * cannot depend on them, and refilling would cost a pass over a 65536 by
     * 762 trace to reach the same answer.
     */
    /*
     * A boundary pins its cell to a constant, which holds it just as firmly as
     * a transition does. Transitions are asked by perturbation; boundaries are
     * read off directly.
     */
    let pinned: BTreeSet<usize> = Air::boundary(&asm.wired)
        .into_iter()
        .filter(|&(col, _, _)| col < width)
        .map(|(col, row, _)| row * width + col)
        .collect();
    println!("{} wired cells are pinned by a boundary", wired.intersection(&pinned).count());

    let t = Instant::now();
    let mut free: BTreeSet<usize> = BTreeSet::new();
    let mut trace = asm.witness.clone();
    for &cell in &wired {
        if pinned.contains(&cell) {
            continue;
        }
        let (r, c) = (cell / width, cell % width);
        if r + 1 >= total {
            continue;
        }
        let idx = r * stride + c;
        let saved = trace[idx];
        trace[idx] = saved + Fp::ONE;
        /*
         * A cell at row r appears in the window opening at r and, when r is
         * not the first row, in the one opening at r - 1. No other window can
         * see it, so no other window can object.
         */
        let quiet = regions_only(&at(&trace, r)) && (r == 0 || regions_only(&at(&trace, r - 1)));
        trace[idx] = saved;
        if quiet {
            free.insert(cell);
        }
    }
    println!("probed {} cells in {:?}", wired.len(), t.elapsed());

    /*
     * One free cell in a class is enough to break that class: move it and it
     * no longer equals the cells it is wired to, whether or not they are free
     * themselves. So the forgery wants two classes with a free cell each, one
     * to break and one to pay with, and not two wholly free classes. Counting
     * the wholly free ones is the question a careless reading asks, and on
     * this circuit it answers zero, which is not an all clear.
     */
    let with_free: Vec<&Vec<usize>> = classes
        .iter()
        .filter(|c| c.iter().any(|cell| free.contains(cell)))
        .collect();
    let wholly_free = classes
        .iter()
        .filter(|c| c.iter().all(|cell| free.contains(cell)))
        .count();
    println!(
        "\ncells held only by the wiring: {} of {}",
        free.len(),
        wired.len()
    );
    println!(
        "classes with a cell held only by the wiring: {} of {}",
        with_free.len(),
        classes.len()
    );
    println!("classes entirely so: {wholly_free}");
    for c in with_free.iter().take(6) {
        let cols: Vec<usize> = c.iter().map(|cell| cell % width).collect();
        let rows: Vec<usize> = c.iter().map(|cell| cell / width).collect();
        let n = c.iter().filter(|cell| free.contains(cell)).count();
        println!(
            "  class of {} cells, {n} free, columns {cols:?}, rows {rows:?}",
            c.len()
        );
    }

    println!(
        "\n{}",
        if with_free.len() >= 2 {
            "two or more classes carry a cell nothing but the product holds, so \
             the forgery applies to this circuit: break one, solve the other \
             for the cell that pays, and every region still agrees"
        } else if with_free.len() == 1 {
            "one class carries a free cell. A forgery needs a second to pay \
             with, so the two class construction does not close here, and the \
             argument is still being made at a point the prover knows"
        } else {
            "every wired cell is also held by some region's own rules, so the \
             fixed challenges are not reachable through this wiring. The \
             argument is still unsound and any new region with witness cells \
             reopens it"
        }
    );
}
