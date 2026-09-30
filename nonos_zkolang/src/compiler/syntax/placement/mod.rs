/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The placement check, run on the tree once a file is parsed: `return`, `break`,
 * `continue` and assignment may stand only where a statement may.
 */

mod check;
mod control;
mod expr;
mod items;
mod types;

pub(super) use items::check_items;
