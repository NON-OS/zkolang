/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The per-program verifier key.

use alloc::vec::Vec;

use nonos_stark::air::{periodic_root as air_periodic_root, Air};
use nonos_stark::hash::keccak256;

use super::air_for::air_for;
use super::{KeyError, WIRING_VERSION};
use crate::air::TRACE_WIDTH;
use crate::commit;
use crate::isa::Op;

/// `keccak256(descriptor ‖ periodic_root)`. The descriptor binds the wiring version,
/// the program's 32-byte commitment, the padded trace length, the trace width, and
/// the FRI rate. A challenger recomputes both halves and rejects a wrong entry.
pub fn verifier_key(program: &[Op], extra_blowup_bits: u32) -> Result<[u8; 32], KeyError> {
    let air = air_for(program)?;
    let root = air_periodic_root(&air, extra_blowup_bits);
    let mut buf: Vec<u8> = Vec::new();
    buf.push(WIRING_VERSION);
    buf.extend_from_slice(&commit::commit(program));
    buf.extend_from_slice(&air.log_trace_len().to_le_bytes());
    buf.extend_from_slice(&(TRACE_WIDTH as u32).to_le_bytes());
    buf.extend_from_slice(&extra_blowup_bits.to_le_bytes());
    buf.extend_from_slice(&root);
    Ok(keccak256(&buf))
}
