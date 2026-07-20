/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Price a proving run from its report.

use super::{Quote, BASE_MICRONOX, MICRONOX_PER_KCELL, PROTOCOL_FEE_BPS};
use crate::driver::Report;

/// Price a proving run: price follows work, so the quote is agreed before a prover
/// commits. The report's trace shape is the whole input.
pub fn quote(report: &Report) -> Quote {
    let cells = report.trace_len as u64 * report.trace_width as u64;
    let compute_micronox = cells.saturating_mul(MICRONOX_PER_KCELL) / 1_000;
    let total_micronox = BASE_MICRONOX.saturating_add(compute_micronox);
    let protocol_fee_micronox = total_micronox.saturating_mul(PROTOCOL_FEE_BPS) / 10_000;
    let prover_micronox = total_micronox - protocol_fee_micronox;
    Quote {
        cells,
        base_micronox: BASE_MICRONOX,
        compute_micronox,
        total_micronox,
        protocol_fee_micronox,
        prover_micronox,
    }
}
