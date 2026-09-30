/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Checking a program: its crates collected, then every check over them. */

mod check;
mod collect;
mod run;

pub use check::{check, check_crates, check_tests};
