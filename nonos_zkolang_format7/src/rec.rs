/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A field element that records how it was made. The step AIR's transition is written
 * once over any field; run over this one, it leaves on a tape the additions,
 * subtractions and multiplications it performs, and that tape is the constraint program
 * a program image carries. The tape belongs to the thread that records it.
 */

use std::cell::RefCell;
use std::collections::BTreeMap;

use nonos_stark::field::{Fp, Fp2};

/** One operation of a tape, over the handles of earlier ones. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    /** A constant of the extension field, as its two base components. */
    Const(Fp, Fp),
    /** Input `k`: a cell of the frame, then a periodic value. */
    Input(u32),
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
}

/** The operations recorded so far, and whether an inversion was asked for. */
#[derive(Default)]
pub(crate) struct Tape {
    pub(crate) ops: Vec<Op>,
    consts: BTreeMap<(u64, u64), u32>,
    pub(crate) inverted: bool,
}

thread_local! {
    pub(crate) static TAPE: RefCell<Tape> = RefCell::new(Tape::default());
}

/** A handle onto this thread's tape. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rec(pub(crate) u32);

/** Record `op` and hand back its handle. */
pub(crate) fn push(op: Op) -> Rec {
    TAPE.with(|t| {
        let mut t = t.borrow_mut();
        t.ops.push(op);
        Rec(t.ops.len() as u32 - 1)
    })
}

/** The handle of constant `v`, recorded once however often it is asked for. */
pub(crate) fn constant(v: Fp2) -> Rec {
    let key = (v.c0.to_u64(), v.c1.to_u64());
    if let Some(id) = TAPE.with(|t| t.borrow().consts.get(&key).copied()) {
        return Rec(id);
    }
    let r = push(Op::Const(v.c0, v.c1));
    TAPE.with(|t| t.borrow_mut().consts.insert(key, r.0));
    r
}
