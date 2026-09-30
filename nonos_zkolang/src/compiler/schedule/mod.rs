/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Scheduling: the order of a machine-level SSA program chosen to keep few values in
 * registers at once. The machine has no memory, and a secret input or advice value cannot
 * be read twice, so a value is held from its first read to its last use, and the order
 * decides how many are held together. Two orders are tried, the program's own and one
 * evaluating every constraint and output depth first, and the one holding fewer values
 * at once is kept. In both an input or advice value is read only when needed, and a
 * constraint, an output or an operation freeing a register is placed as soon as it can
 * be. The order changes no value and no constraint.
 */

mod eager;
mod graph;
mod graph_parts;
mod order;
mod peak;
mod rebuild;
mod visit;

pub use rebuild::schedule;
