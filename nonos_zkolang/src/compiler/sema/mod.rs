/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Semantic analysis: the items and names of a program, the types of its expressions, and
 * the typed IR a checked program lowers to.
 */

mod alias;
mod body;
mod calls;
pub mod check;
mod const_arg;
mod const_fns;
mod const_path;
mod const_run;
mod const_ty;
mod consts;
pub mod cx;
pub mod defs;
mod entry;
mod env;
mod eval;
mod info;
mod lower;
mod lower_path;
mod no_generics;
mod not_checked;
mod path_report;
mod prepare;
mod program;
mod program_env;
mod program_failed;
mod recursion;
mod register;
mod scc;
mod signature;
pub mod ty;
mod uses;
mod within;

pub use entry::check;
