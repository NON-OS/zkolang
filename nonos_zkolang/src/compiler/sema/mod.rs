/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Semantic analysis: the items and names of a program, the types of its expressions, and
 * the typed IR a checked program lowers to.
 */

mod adt;
mod adt_decl;
mod adt_enum;
mod adt_fields;
mod adt_known;
mod adt_pass;
mod alias;
pub(crate) mod attr_query;
mod attr_rules;
mod attrs;
mod body;
mod body_fn;
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
mod deprecated;
mod entry;
mod env;
mod eval;
mod exhaust;
mod fn_bound;
mod fn_instance;
mod fn_name;
mod generics_scope;
mod impl_member;
mod impls;
mod info;
mod lints;
mod lower;
mod lower_path;
mod main_check;
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
pub mod secret;
mod self_ty;
mod sig_instance;
mod signature;
mod tests;
pub mod ty;
mod type_args;
mod type_def;
mod type_params;
mod uses;
mod within;

pub use entry::{check, check_tests};
