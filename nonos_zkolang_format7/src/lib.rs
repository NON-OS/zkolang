/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * zKølang programs as STARKs format 7 statements. The step AIR's constraints are recorded
 * once as a tape, and a program's image, the bytes the STARKs verifiers read a circuit
 * from, carries them; a run is proven in format 7 and verified by `nox_verify`, the
 * verifier the STARKs gates, browser and chain verifiers share.
 */

mod air;
mod error;
mod image;
mod image_ops;
mod params;
mod prove;
mod prune;
mod rec;
mod rec_ops;
mod rec_tape;
mod replay;
mod statement;
mod tape;
mod verify;
mod word;

pub use air::Air7;
pub use error::Error;
pub use image::{image, ImageError};
pub use nox_verify::Refusal;
pub use params::{
    blind_degree, min_log_t, EXTRA_BLOWUP_BITS, GRIND_BITS, MAX_PROOF_BYTES, QUERIES,
};
pub use prove::{prove, Proof};
pub use rec::{Op, Rec};
pub use replay::{probe, replay};
pub use statement::{statement, Statement};
pub use tape::{record, Transition};
pub use verify::{shape, verify};
pub use word::{word, words};
