/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One file per command. Each returns an error message, and `main` prints it. */

mod build;
mod check;
mod fee;
mod key;
mod prepare;
mod run;

pub(crate) use build::build;
pub(crate) use check::check;
pub(crate) use fee::fee;
pub(crate) use key::key;
pub(crate) use run::run;
