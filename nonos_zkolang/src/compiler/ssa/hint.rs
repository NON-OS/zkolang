/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * How the prover computes an advice value from values computed before it. Every advice
 * value is pinned by constraints to exactly one choice (section 21.3); the hint only
 * finds that choice, so a wrong hint makes a run unprovable, never unsound.
 */

use super::V;

/** The computation of one advice value. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Hint {
    /** Bit `k` of the canonical representative of `v`. */
    Bit(V, u8),
    /** The quotient of the canonical integers `a / b`, rounded down; 0 when `b = 0`. */
    Quot(V, V),
    /** The remainder of the canonical integers `a mod b`; `a` when `b = 0`. */
    Rem(V, V),
    /** `v` itself: a value kept in the advice while no register holds it. */
    Copy(V),
}
