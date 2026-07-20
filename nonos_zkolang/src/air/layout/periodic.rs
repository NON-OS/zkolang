/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Periodic (public) wiring columns: the write one-hot, then the three read-port
//! one-hots, each `REGS` wide.

use crate::isa::REGS;

pub(crate) const WRITE_P: usize = 0;
pub(crate) const READA_P: usize = REGS;
pub(crate) const READB_P: usize = 2 * REGS;
pub(crate) const READC_P: usize = 3 * REGS;
pub(crate) const NUM_PERIODIC: usize = 4 * REGS;
