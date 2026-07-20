/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Check and pad a wiring list into the AIR.

use alloc::vec::Vec;

use super::error::BuildError;
use super::step_air::StepAir;
use super::wiring::WireRow;

/// Reject an unhalted or over-long run, pad the wiring to the power-of-two length,
/// and build the AIR. The padding and checks are identical for both entry points.
pub(super) fn pad_and_build(
    mut wiring: Vec<WireRow>,
    halted: bool,
    log_t: u32,
    public_bindings: Vec<(usize, usize, nonos_stark::field::Fp)>,
) -> Result<StepAir, BuildError> {
    if !halted {
        return Err(BuildError::NoHalt);
    }
    let t = 1usize << log_t;
    if wiring.len() > t {
        return Err(BuildError::TooLong {
            rows: wiring.len(),
            cap: t,
        });
    }
    while wiring.len() < t {
        wiring.push(WireRow::EMPTY);
    }
    Ok(StepAir {
        log_t,
        wiring,
        public_bindings,
    })
}
