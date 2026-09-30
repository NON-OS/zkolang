/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One file per command. Each returns an error message, and `main` prints it. */

mod build;
mod check;
mod check_2026;
mod disk;
mod doc;
mod edition;
mod explain;
mod fee;
mod key;
mod modern;
mod prepare;
mod run;
mod run_2026;
mod test;
mod test_report;
mod values;

pub(crate) use build::build;
pub(crate) use check::check;
pub(crate) use doc::doc;
pub(crate) use explain::explain;
pub(crate) use fee::fee;
pub(crate) use key::key;
pub(crate) use run::run;
pub(crate) use test::test;
