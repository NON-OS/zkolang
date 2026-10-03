/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One file per command. Each returns an error message, and `main` prints it. */

mod abi;
mod abi_words;
mod build;
mod check;
mod check_2026;
mod cost;
mod disk;
mod doc;
mod edition;
mod explain;
mod fee;
mod fmt;
mod format7;
mod format7_why;
mod key;
mod library;
mod modern;
mod prepare;
mod prove7;
mod run;
mod run_2026;
mod statement;
mod test;
mod test_report;
mod values;
mod verify7;

pub(crate) use abi::abi;
pub(crate) use build::build;
pub(crate) use check::check;
pub(crate) use disk::manifest_for;
pub(crate) use doc::doc;
pub(crate) use explain::explain;
pub(crate) use fee::fee;
pub(crate) use fmt::fmt;
pub(crate) use key::key;
pub(crate) use prove7::prove;
pub(crate) use run::run;
pub(crate) use statement::statement;
pub(crate) use test::test;
pub(crate) use verify7::verify;
