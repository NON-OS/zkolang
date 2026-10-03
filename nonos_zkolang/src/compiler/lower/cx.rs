/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The state of lowering. `g` is the guard: a boolean value that is 1 exactly when the
 * point being lowered runs (section 8.4). Every assignment is `Sel(g, new, old)`, every
 * failure condition is conditioned on `g`, and an exit sets `g` to 0, so the arms of an
 * `if` are lowered one after the other on one state, with no merging.
 */

use alloc::vec::Vec;

use super::sites::Sites;
use crate::compiler::ssa::{Builder, V};
use crate::compiler::tir::{FnId, TProgram};

/** One loop being lowered: the guards of the `break`s and `continue`s met so far. */
#[derive(Clone, Copy, Debug)]
pub(super) struct LoopCx {
    pub(super) broke: V,
    pub(super) cont: V,
}

/** One call being lowered: its function, locals, result so far, and loops. */
#[derive(Clone, Debug)]
pub(super) struct Frame {
    pub(super) f: FnId,
    /** Each local's slots as they now stand. */
    pub(super) locals: Vec<Vec<V>>,
    /** The value returned by a `return` passed so far, selected by its guard. */
    pub(super) ret: Vec<V>,
    pub(super) loops: Vec<LoopCx>,
}

/** Lowering a program. */
pub(super) struct Lower<'p> {
    pub(super) p: &'p TProgram,
    pub(super) b: Builder,
    pub(super) g: V,
    pub(super) frames: Vec<Frame>,
    /** The sites of the instructions written so far. */
    pub(super) sites: Sites,
}

impl<'p> Lower<'p> {
    /** The frame of the call being lowered. */
    pub(super) fn frame(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut()
    }

    /** The innermost loop of the call being lowered. */
    pub(super) fn innermost(&mut self) -> Option<&mut LoopCx> {
        self.frames.last_mut().and_then(|f| f.loops.last_mut())
    }
}
