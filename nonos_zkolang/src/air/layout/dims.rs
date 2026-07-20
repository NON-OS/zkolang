/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The AIR's dimensions: the window, the transition-constraint count, and the degree.

use crate::isa::REGS;

/// The window is a row and its successor, so ordering and write propagation can read
/// the next row.
pub(crate) const WINDOW: usize = 2;

/// Transition constraints: 27 step constraints, three read bindings, and one write
/// propagation per register.
pub(crate) const NUM_TRANSITION: usize = 27 + 3 + REGS;

/// Highest constraint degree, from the multiply gate and the witnessed equality. The
/// register bindings are linear.
pub(crate) const DEGREE: usize = 3;
