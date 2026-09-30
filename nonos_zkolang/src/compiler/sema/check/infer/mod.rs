/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Type inference in a body: its type variables, unifying types, and resolving them. */

mod occurs;
mod unify;
mod unify_adt;
mod vars;
mod vars_general;
mod vars_join;
mod vars_query;
mod vars_root;
mod zonk;
mod zonk_adt;

pub(crate) use vars::IntVars;
