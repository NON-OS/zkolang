/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Why a format 7 statement or proof could not be made, in words. */

use nonos_zkolang::compiler::driver::abi::AbiError;
use nonos_zkolang_format7::{Error, ImageError};

/** Why no statement or proof was made, other than a failing run. */
pub(super) fn why(e: &Error) -> String {
    match e {
        Error::Run(f) => format!("the run was not proved: {f:?}"),
        Error::Shape => {
            String::from("the program has no halt or needs more rows than a trace holds")
        }
        Error::Image(ImageError::TooLarge(what)) => {
            format!("the program image has {what} past the format's bound")
        }
        Error::Transition => String::from("the step AIR did not record as a tape"),
        Error::Prover => String::from("the prover stopped"),
        Error::Unverified(why) => format!("the proof made does not verify: {why}"),
    }
}

/** Why the values given for `flag` do not fit. */
pub(super) fn abi_why(flag: &str, e: AbiError) -> String {
    match e {
        AbiError::Count { expected, got } => {
            let n = |k: usize| format!("{k} value{}", if k == 1 { "" } else { "s" });
            format!("{flag}: {} needed, {} given", n(expected), n(got))
        }
        AbiError::Range { position } => {
            format!("{flag}: value {} is not a value of its type", position + 1)
        }
    }
}
