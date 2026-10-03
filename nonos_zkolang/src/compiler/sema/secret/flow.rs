/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The walk of one function's body that follows labels (section 13.1). It keeps each
 * local's labels as the body runs, the label of the guard the current point runs under,
 * and what leaves the function or a loop under which guard.
 */

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use super::shape::Shape;
use super::summary::Summary;
use super::taint::Taint;
use crate::compiler::diag::Diagnostic;
use crate::compiler::tir::{TFn, TProgram};

/** The state of the walk. */
pub(super) struct Flow<'p> {
    pub(super) program: &'p TProgram,
    pub(super) summaries: &'p [Option<Summary>],
    pub(super) f: &'p TFn,
    pub(super) env: Vec<Shape>,
    /** The label of the guard the current point runs under (section 8.4). */
    pub(super) pc: Taint,
    /** The values returned so far, each raised by its guard. */
    pub(super) returned: Shape,
    /** The guards of the `return`s so far. */
    pub(super) ret_guard: Taint,
    /** Each `&mut` parameter's local, and the labels it has had at a `return`. */
    pub(super) ref_ret: Vec<(usize, Shape)>,
    /** For each open loop, the guards of what left it: `break`, `continue`, `return`. */
    pub(super) exits: Vec<Taint>,
    /** The slots whose arguments must be public, as bits. */
    pub(super) needs_public: u128,
    /** Whether to report, which the passes of a loop before its fixed point do not. */
    pub(super) report: bool,
    pub(super) reported: BTreeSet<(u32, u32)>,
    pub(super) found: Vec<Diagnostic>,
}
