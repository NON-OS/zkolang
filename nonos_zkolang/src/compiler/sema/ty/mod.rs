/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Types: the kinds a type takes, and the table that gives each distinct type one id. */

mod display;
mod kind;
mod prim;
mod types;

pub use kind::{TyId, TyKind};
pub use types::Types;
