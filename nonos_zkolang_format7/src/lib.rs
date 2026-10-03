/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * zKølang programs as STARKs format 7 statements: the step AIR's constraints recorded
 * once as a tape, and a program's image, the bytes the STARKs verifiers read a circuit
 * from.
 */

mod image;
mod image_ops;
mod prune;
mod rec;
mod rec_ops;
mod rec_tape;
mod replay;
mod tape;
mod word;

pub use image::{image, ImageError};
pub use rec::{Op, Rec};
pub use replay::{probe, replay};
pub use tape::{record, Transition};
pub use word::word;
