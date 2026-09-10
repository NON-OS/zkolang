// NONOS Operating System (AGPL-3.0-or-later)
//! The recording is faithful or the strip is fiction: run the real inner's
//! transition over the tape, replay the tape over random values, and the
//! replayed outputs must equal the direct evaluation's, output for output.

use crate::compose_pipeline::{begin, eval, mul_count, snapshot, Cell};
use crate::crypto::stark::air::GenericTransition;
use crate::crypto::stark::field::{Ext2, Fp};

fn random_fps(n: usize, seed: u64) -> Vec<Fp> {
    // A fixed-seed LCG: deterministic test values, no rng dependency.
    let mut s = seed;
    (0..n)
        .map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            Fp::from_u64(s >> 11)
        })
        .collect()
}

fn tape_matches_direct<A: GenericTransition>(air: &A, w: usize, p: usize, seed: u64) {
    // Record: 2 base lanes per Ext2 value, frame then periodic.
    let cells = begin(2 * (w + p));
    let frame: Vec<Ext2<Cell>> =
        (0..w).map(|i| Ext2::new(cells[2 * i], cells[2 * i + 1])).collect();
    let per: Vec<Ext2<Cell>> = (0..p)
        .map(|i| Ext2::new(cells[2 * (w + i)], cells[2 * (w + i) + 1]))
        .collect();
    let out_cells = air.transition_gen::<Ext2<Cell>>(&frame, &per);
    let tape = snapshot();

    // Replay over random values and evaluate directly over the same values.
    let inputs = random_fps(2 * (w + p), seed);
    let vals = eval(&tape, &inputs);
    let frame_v: Vec<Ext2<Fp>> =
        (0..w).map(|i| Ext2::new(inputs[2 * i], inputs[2 * i + 1])).collect();
    let per_v: Vec<Ext2<Fp>> = (0..p)
        .map(|i| Ext2::new(inputs[2 * (w + i)], inputs[2 * (w + i) + 1]))
        .collect();
    let direct = air.transition_gen::<Ext2<Fp>>(&frame_v, &per_v);

    assert_eq!(out_cells.len(), direct.len());
    for (i, (oc, dv)) in out_cells.iter().zip(&direct).enumerate() {
        let replayed = Ext2::new(vals[oc.c0.0 as usize], vals[oc.c1.0 as usize]);
        assert!(replayed == *dv, "output {i} diverges between tape and direct");
    }
    std::println!(
        "tape: {} nodes, {} muls, {} outputs",
        tape.len(),
        mul_count(&tape),
        direct.len()
    );
}

/// The strip plan for the real tape: every mul scheduled at its level, every
/// producer's liveness reaching its consumers, the window-2 audit green, and
/// the width pressure printed so the budget is a number, not a hope.
#[test]
#[ignore]
fn the_real_tape_schedules() {
    use crate::compose_pipeline::{audit, plan};
    let js = crate::shield_deployed_wired();
    let w = 2 * crate::crypto::stark::air::Air::trace_width(&js);
    let p = crate::crypto::stark::air::Air::periodic_columns(&js).len();
    let cells = begin(2 * (w + p));
    let frame: Vec<Ext2<Cell>> =
        (0..w).map(|i| Ext2::new(cells[2 * i], cells[2 * i + 1])).collect();
    let per: Vec<Ext2<Cell>> = (0..p)
        .map(|i| Ext2::new(cells[2 * (w + i)], cells[2 * (w + i) + 1]))
        .collect();
    let outs = js.transition_gen::<Ext2<Cell>>(&frame, &per);
    let tape = snapshot();
    let out_ids: Vec<u32> = outs.iter().flat_map(|o| [o.c0.0, o.c1.0]).collect();

    let pl = plan(&tape, &out_ids);
    assert!(audit(&tape, &pl), "a scheduled operand escaped its window");
    std::println!(
        "strip: {} muls over {} rows, peak carry {}, inputs {}, width pressure {}",
        pl.slots.len(),
        pl.rows,
        pl.peak_carry,
        pl.n_inputs,
        pl.n_inputs + 2 * pl.peak_carry
    );

    let wm = crate::compose_pipeline::witnessed_muls(&tape);
    std::println!("witnessed (var x var) muls: {} of {}", wm.len(), 3525);

    for k in [2usize, 4, 8, 16, 32] {
        let pk = crate::compose_pipeline::pack(&tape, &out_ids, k);
        std::println!(
            "pack k={k}: rows={} echo={} width={} (carry={} inputs={} fed={})",
            pk.rows,
            pk.peak_echo,
            pk.width,
            pk.peak_carry,
            pk.peak_inputs,
            pk.n_output_fed
        );
    }
}

/// The deployed join-split: the inner the settlement recursion carries.
/// Release-gated like the other real-inner walks; the debug build trips a
/// pre-existing debug assert inside the deployed assembly's construction.
#[test]
#[ignore]
fn the_real_inner_records_faithfully() {
    let js = crate::shield_deployed_wired();
    let w = 2 * crate::crypto::stark::air::Air::trace_width(&js);
    let p = crate::crypto::stark::air::Air::periodic_columns(&js).len();
    for seed in [7u64, 40499, 991177] {
        tape_matches_direct(&js, w, p, seed);
    }
}

/// The layout gate: evaluate every lane's linear forms and the accumulator
/// schedule against the replayed tape. Product by product, output by output,
/// the layout must reproduce the recording it was compiled from.
#[test]
#[ignore]
fn the_strip_layout_evaluates() {
    use crate::compose_pipeline::{strip_layout, Source};
    let js = crate::shield_deployed_wired();
    let w = 2 * crate::crypto::stark::air::Air::trace_width(&js);
    let p = crate::crypto::stark::air::Air::periodic_columns(&js).len();
    let cells = begin(2 * (w + p));
    let frame: Vec<Ext2<Cell>> =
        (0..w).map(|i| Ext2::new(cells[2 * i], cells[2 * i + 1])).collect();
    let per: Vec<Ext2<Cell>> = (0..p)
        .map(|i| Ext2::new(cells[2 * (w + i)], cells[2 * (w + i) + 1]))
        .collect();
    let outs = js.transition_gen::<Ext2<Cell>>(&frame, &per);
    let tape = snapshot();
    let out_ids: Vec<u32> = outs.iter().flat_map(|o| [o.c0.0, o.c1.0]).collect();

    let k = 32usize;
    let lay = strip_layout(&tape, &out_ids, k);
    let inputs = random_fps(2 * (w + p), 40499);
    let vals = eval(&tape, &inputs);

    let resolve = |row: usize, s: &Source, lay: &crate::compose_pipeline::StripLayout| -> Fp {
        match s {
            Source::Slot { rel, lane } => {
                let r = if *rel == 1 { row } else { row - 1 };
                vals[lay.rows[r].lanes[*lane as usize].node as usize]
            }
            Source::Echo { idx } => vals[lay.rows[row].echoes[*idx as usize] as usize],
        }
    };

    let mut checked = 0usize;
    let mut max_echo = 0usize;
    for (r, row) in lay.rows.iter().enumerate() {
        max_echo = max_echo.max(row.echoes.len());
        for lane in &row.lanes {
            let ev = |f: &crate::compose_pipeline::LinForm| -> Fp {
                let mut acc = f.constant;
                for (c, s) in &f.terms {
                    acc = acc + *c * resolve(r, s, &lay);
                }
                acc
            };
            let a = ev(&lane.a);
            let b = ev(&lane.b);
            assert!(a * b == vals[lane.node as usize], "lane {} row {r} lies", lane.node);
            checked += 1;
        }
    }

    for (j, out) in out_ids.iter().enumerate() {
        let mut acc = lay.out_consts[j];
        for (r, c, s) in &lay.out_terms[j] {
            acc = acc + *c * resolve(*r, s, &lay);
        }
        assert!(acc == vals[*out as usize], "output {j} diverges");
    }
    std::println!(
        "layout: {} lanes checked over {} rows, max echoes {}, outputs {} exact",
        checked,
        lay.rows.len(),
        max_echo,
        out_ids.len()
    );
}

/// The operand-width census: how many sources each linear form reads decides
/// the term-cell budget T and how many wide forms split into partial sums.
#[test]
#[ignore]
fn the_operand_widths_measure() {
    use crate::compose_pipeline::strip_layout;
    let js = crate::shield_deployed_wired();
    let w = 2 * crate::crypto::stark::air::Air::trace_width(&js);
    let p = crate::crypto::stark::air::Air::periodic_columns(&js).len();
    let cells = begin(2 * (w + p));
    let frame: Vec<Ext2<Cell>> =
        (0..w).map(|i| Ext2::new(cells[2 * i], cells[2 * i + 1])).collect();
    let per: Vec<Ext2<Cell>> = (0..p)
        .map(|i| Ext2::new(cells[2 * (w + i)], cells[2 * (w + i) + 1]))
        .collect();
    let outs = js.transition_gen::<Ext2<Cell>>(&frame, &per);
    let tape = snapshot();
    let out_ids: Vec<u32> = outs.iter().flat_map(|o| [o.c0.0, o.c1.0]).collect();
    let lay = strip_layout(&tape, &out_ids, 32);

    let mut hist: std::collections::BTreeMap<usize, usize> = Default::default();
    let mut consts = 0usize;
    for row in &lay.rows {
        for lane in &row.lanes {
            for f in [&lane.a, &lane.b] {
                *hist.entry(f.terms.len()).or_insert(0) += 1;
                if f.constant != Fp::ZERO {
                    consts += 1;
                }
            }
        }
    }
    let max_out = lay.out_terms.iter().map(|t| t.len()).max().unwrap_or(0);

    // The wide forms: where do their sources live relative to the lane's row?
    let mut same_window = 0usize;
    let mut spread = 0usize;
    let mut echo_heavy = 0usize;
    for row in &lay.rows {
        for lane in &row.lanes {
            for f in [&lane.a, &lane.b] {
                if f.terms.len() < 8 {
                    continue;
                }
                let echoes = f
                    .terms
                    .iter()
                    .filter(|(_, s)| matches!(s, crate::compose_pipeline::Source::Echo { .. }))
                    .count();
                if echoes == 0 {
                    same_window += 1;
                } else if echoes * 2 < f.terms.len() {
                    spread += 1;
                } else {
                    echo_heavy += 1;
                }
            }
        }
    }
    std::println!(
        "wide forms: {} window-only, {} mixed, {} echo-heavy",
        same_window, spread, echo_heavy
    );

    // One wide form dissected: are its echo sources inputs or products?
    'outer: for row in &lay.rows {
        for lane in &row.lanes {
            for f in [&lane.a, &lane.b] {
                if f.terms.len() != 33 {
                    continue;
                }
                let mut inputs = 0usize;
                let mut products = 0usize;
                for (_, src) in &f.terms {
                    if let crate::compose_pipeline::Source::Echo { idx } = src {
                        let id = row.echoes[*idx as usize];
                        match &tape[id as usize] {
                            crate::compose_pipeline::Node::Input(_) => inputs += 1,
                            _ => products += 1,
                        }
                    }
                }
                std::println!(
                    "a 33-form: {} input echoes, {} product echoes, {} window terms",
                    inputs, products, 33 - inputs - products
                );
                break 'outer;
            }
        }
    }
    std::println!("operand terms histogram: {hist:?}");
    std::println!("operands with nonzero constant: {consts}");
    std::println!("widest output form: {max_out} terms");
}

/// The region gate: the ComposeStrip built from the emitted plan satisfies
/// its own transition, periodic schedule, and boundary over the real tape's
/// witness, the final accumulators plus the statement parts reproduce every
/// output, and one bent schedule coefficient is named by anchor and lane.
#[test]
#[ignore]
fn the_strip_region_satisfies() {
    use crate::compose_pipeline::{check_region, strip_layout, strip_plan};
    let js = crate::shield_deployed_wired();
    let w = 2 * crate::crypto::stark::air::Air::trace_width(&js);
    let p = crate::crypto::stark::air::Air::periodic_columns(&js).len();
    let cells = begin(2 * (w + p));
    let frame: Vec<Ext2<Cell>> =
        (0..w).map(|i| Ext2::new(cells[2 * i], cells[2 * i + 1])).collect();
    let per: Vec<Ext2<Cell>> = (0..p)
        .map(|i| Ext2::new(cells[2 * (w + i)], cells[2 * (w + i) + 1]))
        .collect();
    let outs = js.transition_gen::<Ext2<Cell>>(&frame, &per);
    let tape = snapshot();
    let out_ids: Vec<u32> = outs.iter().flat_map(|o| [o.c0.0, o.c1.0]).collect();

    let lay = strip_layout(&tape, &out_ids, 4);
    let (plan, stmt) = strip_plan(&tape, &lay);
    std::println!(
        "region: {} rows (padded {}), width {}, {} periodic columns",
        plan.rows.len(),
        1usize << {
            let mut lg = 1u32;
            while (1usize << lg) < plan.rows.len() { lg += 1; }
            lg
        },
        plan.k + plan.echo_width + plan.n_out,
        2 * (plan.candidates() + 1) * plan.k + plan.n_out * plan.k
    );
    for seed in [991177u64, 7] {
        let inputs = random_fps(2 * (w + p), seed);
        assert!(
            check_region(&tape, &plan, &stmt, &inputs, &out_ids).is_none(),
            "an honest witness violated the region"
        );
    }

    // Provoked: bend one schedule coefficient; the region must name it.
    let inputs = random_fps(2 * (w + p), 991177);
    let mut bent = plan.clone();
    'bend: for row in bent.rows.iter_mut() {
        for (l, (a, _)) in row.ops.iter_mut().enumerate() {
            if row.lanes[l] != crate::crypto::stark::air::EMPTY {
                for c in a.coeffs.iter_mut() {
                    if *c != Fp::ZERO {
                        *c = *c + Fp::ONE;
                        break 'bend;
                    }
                }
            }
        }
    }
    let miss = check_region(&tape, &bent, &stmt, &inputs, &out_ids);
    assert!(miss.is_some(), "a bent schedule satisfied the region");
    std::println!("bent schedule named at {:?}", miss.unwrap());
}

/// The strip-mode flat region: recompute out, pins in. Built over the real
/// inner with honest values, every constraint at the single anchor must
/// vanish, acc cells carrying out minus statement by construction.
#[test]
#[ignore]
fn the_strip_mode_flat_satisfies() {
    use crate::compose_pipeline::{strip_layout, strip_plan};
    use crate::crypto::stark::air::{compose_ext, Air, ComposeCheckGen};
    let js = crate::shield_deployed_wired();
    let wdt = crate::crypto::stark::air::Air::trace_width(&js);
    let w = 2 * wdt;
    let p = crate::crypto::stark::air::Air::periodic_columns(&js).len();
    let cells = begin(2 * (w + p));
    let frame_c: Vec<Ext2<Cell>> =
        (0..w).map(|i| Ext2::new(cells[2 * i], cells[2 * i + 1])).collect();
    let per_c: Vec<Ext2<Cell>> = (0..p)
        .map(|i| Ext2::new(cells[2 * (w + i)], cells[2 * (w + i) + 1]))
        .collect();
    let outs = js.transition_gen::<Ext2<Cell>>(&frame_c, &per_c);
    let tape = snapshot();
    let out_ids: Vec<u32> = outs.iter().flat_map(|o| [o.c0.0, o.c1.0]).collect();
    let lay = strip_layout(&tape, &out_ids, 4);
    let (_, stmt) = strip_plan(&tape, &lay);

    use crate::crypto::stark::field::Fp2;
    let inputs = random_fps(2 * (w + p), 40499);
    let frame: Vec<Fp2> =
        (0..w).map(|i| Fp2 { c0: inputs[2 * i], c1: inputs[2 * i + 1] }).collect();
    let periodic: Vec<Fp2> = (0..p)
        .map(|i| Fp2 { c0: inputs[2 * (w + i)], c1: inputs[2 * (w + i) + 1] })
        .collect();
    let nt = js.num_transition();
    let b = js.boundary().len();
    let coeffs: Vec<Fp2> = random_fps(2 * (nt + b), 7)
        .chunks(2)
        .map(|c| Fp2 { c0: c[0], c1: c[1] })
        .collect();
    let zr = random_fps(2, 991177);
    let z = Fp2 { c0: zr[0], c1: zr[1] };
    let g = crate::crypto::stark::fri::root_of_unity(js.log_trace_len());
    let comp_z = compose_ext(&js, g, z, &frame, &periodic, &coeffs);

    let region = ComposeCheckGen::new_witness(
        crate::shield_deployed_wired(),
        frame,
        periodic,
        coeffs,
        z,
        comp_z,
        g,
    )
    .into_strip(stmt);
    let tr = region.trace();
    let wid = region.trace_width();
    let window: Vec<Fp> = tr[..2 * wid].to_vec();
    let res = region.transition(&window, &[]);
    for (i, v) in res.iter().enumerate() {
        assert!(*v == Fp::ZERO, "strip-mode flat constraint {i} does not vanish");
    }
    std::println!(
        "flat strip mode: width {}, {} constraints vanish, degree {}",
        wid,
        res.len(),
        region.constraint_degree()
    );
}

/// The model-to-code bridge for the S-box split. Zkolang.SboxSplit proves over the
/// integers that x4 * x2 * y equals y^7 when x2 = y*y and x4 = x2*x2, and that the honest
/// squares always satisfy the two square constraints. This asserts the real Rust round
/// implements exactly that identity: round_split_generic with the honest squares
/// reproduces the closed round_generic on random states, and its returned square
/// constraints vanish. The Lean identity and the deployed field op now meet in CI, not in
/// anyone's head.
#[test]
fn the_sbox_split_matches_the_lean_model() {
    use crate::crypto::stark::air::{Poseidon, RATE, WIDTH};
    use crate::crypto::stark::field::Fp;

    let h = Poseidon::new(13, [Fp::from_u64(0); RATE]);
    for seed in [1u64, 7, 40499, 991177, 0xdeadbeef] {
        let mut s = seed;
        let mut state = [Fp::ZERO; WIDTH];
        let mut rc = [Fp::ZERO; WIDTH];
        for j in 0..WIDTH {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            state[j] = Fp::from_u64(s >> 11);
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            rc[j] = Fp::from_u64(s >> 11);
        }
        // The honest witness the Lean completeness lemma names: x2 = y*y, x4 = x2*x2.
        let mut x2 = [Fp::ZERO; WIDTH];
        let mut x4 = [Fp::ZERO; WIDTH];
        for j in 0..WIDTH {
            x2[j] = state[j] * state[j];
            x4[j] = x2[j] * x2[j];
        }
        let closed = h.round_generic::<Fp>(&state, &rc);
        let (split, c2, c4) = h.round_split_generic::<Fp>(&state, &x2, &x4, &rc);
        // Soundness: the split S-box output equals the closed y^7 round, lane for lane.
        assert_eq!(closed, split, "split round diverges from closed round at seed {seed}");
        // Completeness: the honest squares make both constraint vectors vanish.
        for j in 0..WIDTH {
            assert_eq!(c2[j], Fp::ZERO, "x2 constraint nonzero at lane {j}, seed {seed}");
            assert_eq!(c4[j], Fp::ZERO, "x4 constraint nonzero at lane {j}, seed {seed}");
        }
    }
}
