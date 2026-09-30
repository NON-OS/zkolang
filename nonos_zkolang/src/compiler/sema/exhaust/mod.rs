/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Whether the arms of a `match` cover every value of its scrutinee, and which arms no
 * value reaches (section 9.3), by the usefulness of patterns over their constructors.
 */

mod check;
mod domain;
mod matrix;
mod pat;
mod split;
mod useful;
mod useful_ctor;
mod useful_wild;
mod witness;

pub(crate) use check::check_match;
