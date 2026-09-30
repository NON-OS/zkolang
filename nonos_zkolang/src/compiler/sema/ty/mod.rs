/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Types: the kinds a type takes, and the table that gives each distinct type one id. */

mod adt;
mod display;
mod generic_arg;
mod kind;
mod prim;
mod types;
mod types_adt;

pub use adt::{Adt, AdtField, AdtId, AdtVariant, Form};
pub use generic_arg::GenArg;
pub use kind::{TyId, TyKind};
pub use types::Types;
