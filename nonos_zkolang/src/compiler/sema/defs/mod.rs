/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The items of a program and the modules that hold them: each named item gets an id, and
 * each module a namespace mapping names to items, filled from its declarations and its
 * `use` imports.
 */

mod bind;
mod collect;
mod collect_items;
mod crates;
mod def;
mod imports;
mod imports_apply;
mod imports_bind;
mod imports_report;
mod module;
mod near;
mod pending;
mod resolve;
mod table;
mod use_flatten;
mod use_join;
mod visible;

pub use def::{Def, DefId, DefKind};
pub use module::{Binding, BindingKind, Module};
pub use pending::PendingImport;
pub use resolve::PathError;
pub use table::Defs;
