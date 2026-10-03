/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A zKølang program as a format 7 statement: what a verifier pins before it reads a
 * proof. The image and the periodic root are the program's alone, so a gate pins them
 * from the program and never from a prover.
 */

use nonos_stark::air::{periodic_root, Air};
use nonos_stark::field::Fp;
use nonos_stark::hash::keccak256;
use nonos_zkolang::compiler::driver::abi::slots;
use nonos_zkolang::compiler::driver::Built;
use nonos_zkolang::{program_log_t, StepAir};
use stark_proofs::proof_wire::ParamSet;

use super::air::Air7;
use super::error::Error;
use super::image::image;
use super::params::{min_log_t, EXTRA_BLOWUP_BITS, GRIND_BITS, QUERIES};
use super::tape::record;

/** What a verifier of a program's format 7 proofs pins. */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    /** The program image, and its keccak256. */
    pub image: Vec<u8>,
    pub image_hash: [u8; 32],
    /** The root of the program's periodic columns, its wiring. */
    pub periodic_root: [u8; 32],
    /** The trace holds `2^log_t` rows. */
    pub log_t: u32,
    /** The parameter identity of the one point a proof is accepted at. */
    pub params: [u8; 32],
    /** The public words: the commitment limbs, the trace length, the inputs, the outputs. */
    pub words: usize,
}

/** The statement of `b`, from the program alone. */
pub fn statement(b: &Built) -> Result<Statement, Error> {
    let ops = &b.compiled.machine.ops;
    let log_t = program_log_t(ops).ok_or(Error::Shape)?.max(min_log_t());
    let (inputs, outputs) = (slots(&b.public), slots(&b.output));
    let zero = |n: usize| vec![Fp::ZERO; n];
    let air = StepAir::compile(ops, log_t, &zero(inputs), &zero(outputs));
    of_air(&air.map_err(|_| Error::Shape)?, inputs, outputs)
}

/** The statement of `air`, with `inputs` public input slots and `outputs` output slots. */
pub(crate) fn of_air(air: &StepAir, inputs: usize, outputs: usize) -> Result<Statement, Error> {
    let tape = record().ok_or(Error::Transition)?;
    let image = image(&tape, air, inputs).map_err(Error::Image)?;
    let a7 = Air7(air);
    Ok(Statement {
        image_hash: keccak256(&image),
        image,
        periodic_root: periodic_root(&a7, EXTRA_BLOWUP_BITS),
        log_t: air.log_trace_len(),
        params: ParamSet::of(&a7, QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS).id(),
        words: 5 + inputs + outputs,
    })
}
