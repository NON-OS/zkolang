/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The SSA passes. Each keeps the semantics of `ssa::eval` exactly: the same runs accept,
 * with the same outputs, and the same runs fail. Constant folding, value numbering and
 * dead-code removal shrink the program; range-check removal drops a check that facts
 * established before it already imply.
 */

mod bools;
mod bounds;
mod cse;
mod dce;
mod elide;
mod fold;
mod fold_checks;
mod fold_rules;
mod simplify;

pub use cse::cse;
pub use dce::dce;
pub use elide::elide_range_checks;
pub use fold::fold;
pub use simplify::simplify;
