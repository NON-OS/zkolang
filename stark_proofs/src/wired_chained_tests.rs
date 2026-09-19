// NONOS Operating System (AGPL-3.0-or-later)
// The wiring argued as one chained permutation instead of as packed groups.
// The algebra is proven in the engine's chained_product module. What is proven
// here is that the lanes, accumulator columns, sigma placement, selector and
// boundary agree with each other, on an assembly small enough to read a
// failure off.

use crate::crypto::stark::air::{Air, AirExt, GpGroup, WiredMultiExt};
use crate::crypto::stark::field::{Felt, Fp, Fp2};

/// A region whose every column holds its value. Two cells in a column are then
/// genuinely equal, so wiring them is satisfiable, and cells in different
/// columns genuinely differ, so a permutation crossing columns would not
/// close. Otherwise this would test a constant trace, not the argument.
struct Hold {
    width: usize,
    log_t: u32,
    seeds: Vec<Fp>,
}

impl Hold {
    fn body<F: Felt>(&self, window: &[F]) -> Vec<F> {
        (0..self.width)
            .map(|c| window[self.width + c] - window[c])
            .collect()
    }

    fn trace(&self) -> Vec<Fp> {
        let rows = 1usize << self.log_t;
        let mut t = vec![Fp::ZERO; rows * self.width];
        for r in 0..rows {
            for c in 0..self.width {
                t[r * self.width + c] = self.seeds[c];
            }
        }
        t
    }
}

impl AirExt for Hold {
    fn transition_ext(&self, window: &[Fp2], _p: &[Fp2]) -> Vec<Fp2> {
        self.body(window)
    }
}

impl Air for Hold {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }
    fn trace_width(&self) -> usize {
        self.width
    }
    fn window_size(&self) -> usize {
        2
    }
    fn constraint_degree(&self) -> usize {
        1
    }
    fn num_transition(&self) -> usize {
        self.width
    }
    fn transition(&self, window: &[Fp], _p: &[Fp]) -> Vec<Fp> {
        self.body(window)
    }
    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        (0..self.width).map(|c| (c, 0, self.seeds[c])).collect()
    }
}

const W: usize = 20;
const LOG_T: u32 = 3;

fn region() -> Hold {
    Hold {
        width: W,
        log_t: LOG_T,
        seeds: (0..W).map(|c| Fp::from_u64(1_000 + c as u64)).collect(),
    }
}

/// The wiring: in every column, the cell on row 0 is bound to the cell on row
/// 1. Twenty classes of two cells, over twenty columns.
fn global_sigma(total: usize) -> Vec<usize> {
    let mut sigma: Vec<usize> = (0..total * W).collect();
    for c in 0..W {
        sigma[c] = W + c;
        sigma[W + c] = c;
    }
    sigma
}

fn chained() -> WiredMultiExt {
    let total = 1usize << Air::log_trace_len(&region());
    let boxed: Vec<Box<dyn AirExt>> = vec![Box::new(region())];
    let perm = GpGroup {
        wired_cols: (0..W).collect(),
        sigma: global_sigma(total * 2),
        beta: Fp::from_u64(11),
        gamma: Fp::from_u64(13),
    };
    WiredMultiExt::new_kinds_chained(boxed, &[0], perm, Vec::new())
}

/// One group per class, which is what the wiring costs when it is not argued
/// as one permutation. Twenty groups of one column each.
fn packed() -> WiredMultiExt {
    let total = 1usize << Air::log_trace_len(&region());
    let boxed: Vec<Box<dyn AirExt>> = vec![Box::new(region())];
    let groups: Vec<GpGroup> = (0..W)
        .map(|c| {
            let mut sigma: Vec<usize> = (0..total * 2).collect();
            sigma[0] = 1;
            sigma[1] = 0;
            GpGroup {
                wired_cols: vec![c],
                sigma,
                beta: Fp::from_u64(11),
                gamma: Fp::from_u64(13),
            }
        })
        .collect();
    WiredMultiExt::new_kinds_bounded(boxed, &[0], groups, Vec::new())
}

/// Walk the AIR over a trace and return where a constraint or a boundary does
/// not vanish.
fn offences(air: &WiredMultiExt, trace: &[Fp]) -> Vec<(usize, usize, Fp)> {
    let stride = Air::trace_width(air);
    let total = trace.len() / stride;
    let periodic = Air::periodic_columns(air);
    let mut bad = Vec::new();
    for r in 0..total - 1 {
        let window = &trace[r * stride..(r + 2) * stride];
        let p: Vec<Fp> = periodic.iter().map(|col| col[r]).collect();
        for (i, v) in Air::transition(air, window, &p).into_iter().enumerate() {
            if v != Fp::ZERO {
                bad.push((r, i, v));
            }
        }
    }
    for (col, row, want) in Air::boundary(air) {
        let got = trace[row * stride + col];
        if got != want {
            bad.push((row, col, got));
        }
    }
    bad
}

/// The chained argument accepts the trace it builds, constraint for constraint
/// and boundary for boundary.
#[test]
fn the_chained_argument_accepts_its_own_trace() {
    let air = chained();
    let trace = air.trace(&[region().trace()]);
    let bad = offences(&air, &trace);
    assert!(
        bad.is_empty(),
        "the chained AIR rejects its own trace at {:?}",
        &bad[..bad.len().min(4)]
    );
    assert_eq!(
        Air::constraint_degree(&air),
        crate::crypto::stark::air::chained_product::BLOCK + 2,
        "a chained lane is its block width plus the selector"
    );
}

/// The two forms accept the same wiring.
///
/// No saving is asserted, because on a wiring this small there is none: every
/// class sits on its own column, so packing already spends one sigma column
/// per column. The saving comes from columns appearing in many classes with
/// different neighbours, which is a property of the settlement wiring, and
/// `the_settlement_wiring_survives_the_single_argument` measures it there.
#[test]
fn both_forms_accept_the_same_wiring() {
    let (c, p) = (chained(), packed());
    let ct = c.trace(&[region().trace()]);
    let pt = p.trace(&[region().trace()]);
    assert!(
        offences(&c, &ct).is_empty(),
        "the chained AIR rejects its own trace"
    );
    assert!(
        offences(&p, &pt).is_empty(),
        "the packed AIR rejects its own trace"
    );
    assert_eq!(
        Air::periodic_columns(&c).len(),
        Air::periodic_columns(&p).len(),
        "one class per column costs the same either way, so this wiring must tie"
    );
    assert!(
        Air::trace_width(&c) < Air::trace_width(&p),
        "the chained form must still need fewer accumulator columns than one per class"
    );
}

/// A cell that no longer matches the one it is bound to must break the
/// argument, or the tests above would pass on something that proves nothing.
#[test]
fn a_broken_copy_constraint_breaks_a_lane() {
    let air = chained();
    let mut trace = air.trace(&[region().trace()]);
    let stride = Air::trace_width(&air);
    // Row 1, column 0 is bound to row 0, column 0. Move it, then put the
    // products back the way a prover would.
    trace[stride] = trace[stride] + Fp::ONE;
    air.refill_products(&mut trace);
    assert!(
        !offences(&air, &trace).is_empty(),
        "a cell that no longer equals the cell it is wired to satisfied every lane"
    );
}
