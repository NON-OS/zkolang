/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The step AIR's transition as a tape: its inputs, the frame then the periodic values;
 * its operations; and the handles of its constraints, in the order the AIR returns them.
 * The transition never branches on a value, so one tape is the transition at every
 * point, and it reads the program only through the periodic values, so one tape serves
 * every program. It is recorded over a one-instruction program and kept only if its
 * replay gives what the AIR gives.
 */

use nonos_stark::air::{Air, AirExt, GenericTransition};
use nonos_zkolang::{Op as Isa, StepAir};

use super::prune::prune;
use super::rec::{push, Op, Rec};
use super::rec_tape::{fresh, take};
use super::replay::{probe, replay};

/** A recorded transition. */
pub struct Transition {
    pub ops: Vec<Op>,
    pub outputs: Vec<u32>,
    /** The window's cells, row by row. */
    pub n_frame: usize,
    pub n_periodic: usize,
}

/** The step AIR's transition, or `None` if it inverts or its tape does not replay. */
pub fn record() -> Option<Transition> {
    let air = StepAir::for_key(&[Isa::Halt], 1).ok()?;
    let w = air.window_size() * air.trace_width();
    let p = air.periodic_columns().len();
    fresh();
    let ins: Vec<Rec> = (0..w + p).map(|k| push(Op::Input(k as u32))).collect();
    let out = air.transition_gen(&ins[..w], &ins[w..]);
    let tape = take();
    if tape.inverted {
        return None;
    }
    let out: Vec<u32> = out.iter().map(|r| r.0).collect();
    let (ops, outputs) = prune(&tape.ops, &out);
    let t = Transition {
        ops,
        outputs,
        n_frame: w,
        n_periodic: p,
    };
    let (frame, periodic) = probe(w, p);
    let held = replay(&t, &frame, &periodic) == air.transition_ext(&frame, &periodic);
    held.then_some(t)
}
