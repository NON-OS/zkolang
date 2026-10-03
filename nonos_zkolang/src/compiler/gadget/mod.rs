/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Gadget expansion: the gadget-level instructions written out as machine-level ones and
 * advice. It runs in two phases, so range checks can be dropped between them: division
 * becomes advice, range checks and one equation; then every range check and bit becomes
 * a bit decomposition of advice. Each gadget's constraints admit exactly the one advice
 * its hints compute (section 21.3), and hold exactly when the gadget's semantics holds.
 */

mod decompose;
mod divide;
mod expand;
mod field_bits;
mod read_bits;
mod rewrite;

pub use expand::{expand_bits, expand_division};
