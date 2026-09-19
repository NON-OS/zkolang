// NONOS Operating System (AGPL-3.0-or-later)
// The permutation challenges are constants of the circuit. They are set where
// the groups are built, emitted into the layout the verifier is built from,
// and never drawn from a transcript, so the prover knows them before it picks
// its trace. A grand product is a multiset equality checked at one point, and
// it only argues anything when that point is unpredictable. At a known one the
// prover breaks the copy constraints it dislikes and solves for a free cell
// that puts the product back.
//
// So that is built here rather than described, and run against both shapes of
// the argument so neither can be blamed for it.
//
// The fix is a commitment round, not a constant with more entropy: commit the
// region columns, draw beta and gamma from that root, commit the permutation
// columns second. `challenges_drawn_after_the_columns_refuse_the_forgery`
// pins that property. `fixed_challenges_admit_a_compensated_forgery` must
// start failing once the prover draws them, and that failure is the fix
// landing.
//
// Whether the deployed outer has the free cells the forgery needs is a
// question about that circuit, and `free_wired_cells` answers it.

use crate::crypto::stark::air::{Air, AirExt, GpGroup, WiredMultiExt};
use crate::crypto::stark::field::{Fp, Fp2};

/// A region that constrains nothing, which is the shape of every region the
/// recursion holds by wiring rather than by its own rules. Those are the cells
/// a forgery moves.
struct Free {
    width: usize,
    log_t: u32,
}

impl AirExt for Free {
    fn transition_ext(&self, _w: &[Fp2], _p: &[Fp2]) -> Vec<Fp2> {
        Vec::new()
    }
}

impl Air for Free {
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
        0
    }
    fn transition(&self, _w: &[Fp], _p: &[Fp]) -> Vec<Fp> {
        Vec::new()
    }
    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        Vec::new()
    }
}

const W: usize = 4;
const LOG_T: u32 = 2;
/// The challenges the packer bakes into every group of the real circuit.
const BETA: u64 = 5;
const GAMMA: u64 = 7;

fn region() -> Free {
    Free {
        width: W,
        log_t: LOG_T,
    }
}

/// Two classes, each binding a column's row 0 to its row 1: column 0 and
/// column 1. Every other slot is a fixed point, so its factors cancel.
fn classes() -> [(usize, usize); 2] {
    [(0, W), (1, W + 1)]
}

fn sigma(total: usize) -> Vec<usize> {
    let mut s: Vec<usize> = (0..total * W).collect();
    for (a, b) in classes() {
        s[a] = b;
        s[b] = a;
    }
    s
}

fn air(chained: bool) -> WiredMultiExt {
    let total = 1usize << Air::log_trace_len(&region());
    let boxed: Vec<Box<dyn AirExt>> = vec![Box::new(region())];
    let perm = GpGroup {
        wired_cols: (0..W).collect(),
        sigma: sigma(total * 2),
        beta: Fp::from_u64(BETA),
        gamma: Fp::from_u64(GAMMA),
    };
    if chained {
        WiredMultiExt::new_kinds_chained(boxed, &[0], perm, Vec::new())
    } else {
        WiredMultiExt::new_kinds_bounded(boxed, &[0], vec![perm], Vec::new())
    }
}

fn accepts(air: &WiredMultiExt, trace: &[Fp]) -> bool {
    let stride = Air::trace_width(air);
    let total = trace.len() / stride;
    let periodic = Air::periodic_columns(air);
    for r in 0..total - 1 {
        let window = &trace[r * stride..(r + 2) * stride];
        let p: Vec<Fp> = periodic.iter().map(|col| col[r]).collect();
        if Air::transition(air, window, &p)
            .into_iter()
            .any(|v| v != Fp::ZERO)
        {
            return false;
        }
    }
    Air::boundary(air)
        .into_iter()
        .all(|(col, row, want)| trace[row * stride + col] == want)
}

fn linear(v: Fp, slot: usize) -> Fp {
    v + Fp::from_u64(BETA) * Fp::from_u64(slot as u64) + Fp::from_u64(GAMMA)
}

/// The factor a two cycle contributes, as (numerator, denominator).
fn cycle(v_a: Fp, a: usize, v_b: Fp, b: usize) -> (Fp, Fp) {
    (
        linear(v_a, a) * linear(v_b, b),
        linear(v_a, b) * linear(v_b, a),
    )
}

/// Break one copy constraint and pay for it with the other.
///
/// Column 0's bound cells are set unequal. That costs a factor the prover can
/// work out, because it knows beta and gamma, and the equation restoring it is
/// linear in column 1's second cell. Nothing else has an opinion about either.
fn forged_region_trace() -> Vec<Fp> {
    let (a, b) = classes()[0];
    let (c, d) = classes()[1];
    let (v_a, v_b) = (Fp::from_u64(1_000), Fp::from_u64(1_001));
    let v_c = Fp::from_u64(2_000);
    let (num0, den0) = cycle(v_a, a, v_b, b);

    /*
     * With x the free cell, the second cycle contributes
     *   num1 = linear(v_c, c) * (x + beta * d + gamma)
     *   den1 = linear(v_c, d) * (x + beta * c + gamma)
     * and the product closes when num0 * num1 == den0 * den1, which is one
     * linear equation in x.
     */
    let (p, q) = (num0 * linear(v_c, c), den0 * linear(v_c, d));
    let zero = Fp::ZERO;
    let x = (q * linear(zero, c) - p * linear(zero, d)) * (p - q).inv();

    let total = 1usize << LOG_T;
    let mut t = vec![Fp::ZERO; total * W];
    t[a] = v_a;
    t[b] = v_b;
    t[c] = v_c;
    t[d] = x;
    t
}

/// The forgery is accepted, and the copy constraint it breaks is broken.
#[test]
fn fixed_challenges_admit_a_compensated_forgery() {
    let forged = forged_region_trace();
    let (a, b) = classes()[0];
    assert_ne!(
        forged[a], forged[b],
        "the forgery has to break a copy constraint to be a forgery"
    );
    for chained in [false, true] {
        let air = air(chained);
        let trace = air.trace(&[forged.clone()]);
        assert!(
            accepts(&air, &trace),
            "the {} form rejected a trace built to satisfy its own fixed challenges, \
             so either the challenges moved or the forgery is wrong",
            if chained { "chained" } else { "packed" }
        );
    }
}

/// Challenges drawn from the columns they speak about. Keccak stands in for
/// the transcript: all the property needs is that the prover cannot pick its
/// trace against the result.
fn drawn_from(region_trace: &[Fp]) -> (Fp, Fp) {
    let mut bytes = Vec::with_capacity(region_trace.len() * 8);
    for v in region_trace {
        bytes.extend_from_slice(&v.to_u64().to_le_bytes());
    }
    let d = crate::crypto::stark::hash::keccak256(&bytes);
    let word = |i: usize| u64::from_le_bytes(d[i * 8..i * 8 + 8].try_into().unwrap());
    (Fp::from_u64(word(0)), Fp::from_u64(word(1)))
}

/// The same forgery, refused once the challenges come from the trace.
///
/// The forger fixes its region columns, which is what a first round pins, and
/// only then learns where the product is checked. Its compensating cell was
/// solved for somewhere else, so the product does not close.
#[test]
fn challenges_drawn_after_the_columns_refuse_the_forgery() {
    let forged = forged_region_trace();
    let (beta, gamma) = drawn_from(&forged);
    assert!(
        beta != Fp::from_u64(BETA) || gamma != Fp::from_u64(GAMMA),
        "the drawn challenges collided with the baked ones, so this proves nothing"
    );
    for chained in [false, true] {
        let mut air = air(chained);
        air.set_challenges(beta, gamma);
        let trace = air.trace(&[forged.clone()]);
        assert!(
            !accepts(&air, &trace),
            "the {} form accepted the forgery even with the challenges drawn from \
             the columns, so the compensation did not depend on knowing them",
            if chained { "chained" } else { "packed" }
        );
    }
}

/// An honest trace still passes at a drawn point, so the fix refuses
/// forgeries rather than everything.
#[test]
fn an_honest_trace_survives_drawn_challenges() {
    let (a, b) = classes()[0];
    let (c, d) = classes()[1];
    let total = 1usize << LOG_T;
    let mut honest = vec![Fp::ZERO; total * W];
    honest[a] = Fp::from_u64(1_000);
    honest[b] = Fp::from_u64(1_000);
    honest[c] = Fp::from_u64(2_000);
    honest[d] = Fp::from_u64(2_000);
    let (beta, gamma) = drawn_from(&honest);
    for chained in [false, true] {
        let mut air = air(chained);
        air.set_challenges(beta, gamma);
        let trace = air.trace(&[honest.clone()]);
        assert!(
            accepts(&air, &trace),
            "the {} form rejected an honest trace at a drawn point",
            if chained { "chained" } else { "packed" }
        );
    }
}

/// A break with nothing paying for it is still caught, so the argument is not
/// inert. What fails above is checking it somewhere the prover already knew.
#[test]
fn an_uncompensated_break_is_still_caught() {
    let mut broken = forged_region_trace();
    let (_, d) = classes()[1];
    broken[d] = broken[d] + Fp::ONE;
    for chained in [false, true] {
        let air = air(chained);
        let trace = air.trace(&[broken.clone()]);
        assert!(
            !accepts(&air, &trace),
            "a break with no compensating cell was accepted by the {} form",
            if chained { "chained" } else { "packed" }
        );
    }
}
