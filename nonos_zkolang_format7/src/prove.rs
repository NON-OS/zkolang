/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A run proven in STARKs format 7: the step AIR's trace committed in two rounds, every
 * column blinded from the prover's seed, the proof checked against the statement before
 * it is written with its Merkle paths shared.
 */

use nonos_stark::air::{
    blinding_poly, domain_params_blown, share_paths, stark_prove_ext_rounds,
    stark_verify_ext_rounds_positions, Air, Poseidon, RATE,
};
use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::{traced, Built};
use stark_proofs::proof_wire::{serialize_rounds_shared, ParamSet};

use super::air::Air7;
use super::error::Error;
use super::params::QUERIES as Q;
use super::params::{blind_degree, min_log_t, EXTRA_BLOWUP_BITS as E, GRIND_BITS as G};
use super::statement::{of_air, Statement};

/** A format 7 proof, the statement it is of, the public words, and the result. */
pub struct Proof {
    pub bytes: Vec<u8>,
    pub statement: Statement,
    pub words: Vec<u64>,
    pub outputs: Vec<i128>,
}

/**
 * Run `b` on `public` and `secret` leaf values and prove the run in format 7, every trace
 * column blinded with the polynomial expanded from the private `seed`. A fresh seed per
 * proof makes each proof fresh.
 */
pub fn prove(
    b: &Built,
    public: &[i128],
    secret: &[i128],
    seed: &[Fp; RATE],
) -> Result<Proof, Error> {
    let t = traced(b, public, secret, min_log_t()).map_err(Error::Run)?;
    let statement = of_air(&t.air, t.inputs, t.publics.len() - 5 - t.inputs)?;
    let air = Air7(&t.air);
    let params = ParamSet::of(&air, Q, G, E);
    let log_n = domain_params_blown(&air, E).0;
    let h = Poseidon::new(2, [Fp::ZERO; RATE]);
    let deg = blind_degree();
    let blind: Vec<Vec<Fp>> = (0..air.trace_width())
        .map(|c| blinding_poly(&h, seed, c, deg))
        .collect();
    let mut trace = t.trace;
    let proved = stark_prove_ext_rounds(air, &mut trace, Q, G, E, &t.publics, None, &blind);
    let (rounds, _, air) = proved.ok_or(Error::Prover)?;
    let root = &statement.periodic_root;
    let positions = stark_verify_ext_rounds_positions(air, &rounds, Q, G, E, root, &t.publics);
    let positions = positions.map_err(Error::Unverified)?;
    let shared = share_paths(&rounds, &positions, log_n).ok_or(Error::Prover)?;
    Ok(Proof {
        bytes: serialize_rounds_shared(&rounds, &shared, &params),
        statement,
        words: t.publics.iter().map(|f| f.value()).collect(),
        outputs: t.outputs,
    })
}
