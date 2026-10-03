/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Loading a crate's files: its root, then each module in its own file (section 4.2). */

mod entry;
mod files;
mod load_report;
mod splice;

pub use entry::load;
pub use files::{dir_of, join, Files, NoFiles};
