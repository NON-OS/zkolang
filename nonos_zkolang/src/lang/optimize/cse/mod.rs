/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Common-subexpression elimination. A pure subexpression that appears more than once in a
 * statement is computed once into a fresh binding and its occurrences replaced, so the
 * trace holds one copy instead of several. Only pure arithmetic subtrees are shared, so
 * nothing that carries a constraint (an inverse, a division, a comparison, a select) is
 * moved or merged, and the transform cannot change what a program proves. The fresh names
 * use a character a source identifier cannot, so they never collide with a program's own.
 */

mod hoist;
mod ids;
mod pure;
mod rewrite;
mod rewrite_kids;
mod scan;
mod stmts;
mod tag;

pub(super) use stmts::cse;
