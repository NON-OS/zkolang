/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the check of a pattern keeps while it runs. */

use alloc::string::String;
use alloc::vec::Vec;

use crate::compiler::source::Span;
use crate::compiler::tir::LocalId;

/** The state of the pattern being checked. */
#[derive(Default)]
pub(crate) struct PatCx {
    /** Whether it is a `match` arm's, which may fail to match. */
    pub(crate) arm: bool,
    /** Each name it binds so far and its local, in order. */
    pub(crate) bound: Vec<(String, LocalId)>,
    /**
     * For each alternative after the first being checked, innermost last: the names the
     * first alternative binds, their locals, and whether this one binds them yet.
     */
    pub(crate) reuse: Vec<Vec<(String, LocalId, bool)>>,
    /** Where each arm pattern that failed to check is written. */
    pub(crate) broken: Vec<Span>,
}
