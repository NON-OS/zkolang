/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Why a program has no format 7 statement, or a run no format 7 proof. */

use nonos_zkolang::compiler::driver::abi::AbiError;
use nonos_zkolang::compiler::driver::RunFailure;

use super::image::ImageError;

/** Why a statement or a proof could not be made. */
#[derive(Debug)]
pub enum Error {
    /** The run fails, its inputs do not fit, or the compiler and the reference disagree. */
    Run(RunFailure),
    /** The values do not fit the program's inputs or result. */
    Abi(AbiError),
    /** The program has no halt, needs more rows than a trace holds, or will not image. */
    Shape,
    /** The program's image does not fit the format. */
    Image(ImageError),
    /** The transition could not be recorded as a tape. */
    Transition,
    /** The prover stopped. */
    Prover,
    /** The proof made does not verify: a prover bug, never a statement about the run. */
    Unverified(&'static str),
}
