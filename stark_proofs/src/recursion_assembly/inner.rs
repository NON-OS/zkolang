// NONOS Operating System (AGPL-3.0-or-later)
//! The inners a recursion assembles over. `join_split` is the fixture, the
//! regression path `assemble` verifies; `assemble_real` verifies
//! `shield_join_split`, the deployed circuit. The fixture is the wired Accumulator +
//! RangeCheck stand-in, proven Poseidon-committed while absorbing the intent
//! publics, plus the replayed composition inputs every region reads from.

use crate::crypto::stark::air::{
    compose_inputs, compose_inputs_pre, compose_inputs_pub, periodic_root_poseidon,
    stark_prove_poseidon_ext, stark_prove_poseidon_ext_pub, stark_prove_poseidon_pre_pub,
    Accumulator, Air, AirExt, ComposeInputs, PeriodicOpeningP, Poseidon, RangeCheck,
    StarkProofExtP, WiredExt, WiredMultiGen, RATE,
};
use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::fri::root_of_unity;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub const NQ: usize = crate::shield_params::deployment::N_QUERIES;
pub const GRIND: u32 = crate::shield_params::deployment::GRIND_BITS;

/// Deployment blowup, unless the environment lowers it for a wiring gate. The
/// gates test binding logic, which is rate-independent; re-proving the inner
/// at rate 1/16 to check a copy constraint made every debug iteration cost a
/// quarter hour. Emits and vectors never set this.
pub fn extra() -> u32 {
    std::env::var("NONOS_INNER_EXTRA")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(crate::shield_params::deployment::EXTRA_BLOWUP_BITS)
}
pub const EXTRA: u32 = crate::shield_params::deployment::EXTRA_BLOWUP_BITS;

/// The recursion hash. Must equal the round count every in circuit compression
/// runs, or the membership regions prove a permutation the hash never computed.
pub const LOG_ROUNDS: u32 = 5;

pub fn hasher() -> Poseidon {
    Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE])
}

/// The inner proof the assembly verifies, generic over its AIR. The join-split
/// fixture keeps the default `WiredExt`, so the existing assembly is unchanged; a
/// zkolang inner instantiates it at `StepAir`.
pub struct Inner<A: AirExt = WiredExt> {
    pub air: A,
    pub publics: Vec<Fp>,
    pub proof: StarkProofExtP,
    pub ci: ComposeInputs,
    pub t: u64,
    pub g: Fp,
    /// The preprocessed sidecar, when the inner proved against a baked
    /// periodic root. The fixture and step inners stay on the plain path and
    /// carry none.
    pub sidecar: Option<Sidecar>,
}

/// What a preprocessed inner adds: the claims, the opened rows, and the root
/// the outer carries as a constant.
pub struct Sidecar {
    pub periodic_z: Vec<Fp2>,
    pub openings: Vec<PeriodicOpeningP>,
    pub root: [Fp; RATE],
}

/// The fixture air, witness and publics alone, for gates that need the
/// circuit without a proof. `join_split` builds on this, so there is one
/// fixture and it cannot drift from itself.
pub fn join_split_fixture() -> (WiredExt, Vec<Fp>, Vec<Fp>) {
    let (words, k_intents) = (11usize, 2usize);
    let mut publics = Vec::with_capacity(k_intents * words);
    for i in 0..k_intents * words {
        publics.push(Fp::from_u64(0xA000 + i as u64));
    }

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sig: Vec<usize> = (0..32).collect();
    sig.swap(1, 8);
    let air = WiredExt::new(
        regions,
        alloc::vec![0],
        sig,
        Fp::from_u64(5),
        Fp::from_u64(7),
    );

    let neg = |x: u64| Fp::ZERO - Fp::from_u64(x);
    let addends = [
        Fp::from_u64(7),
        Fp::from_u64(3),
        neg(8),
        neg(1),
        neg(1),
        Fp::ZERO,
        Fp::ZERO,
        Fp::ZERO,
    ];
    let mut cons = Vec::new();
    let mut acc = Fp::ZERO;
    for &a in &addends {
        cons.push(acc);
        cons.push(a);
        acc = acc + a;
    }
    let mut rng = Vec::new();
    let mut v = 7u64;
    for i in 0..16usize {
        let bit = if i < 15 { v & 1 } else { 0 };
        rng.push(Fp::from_u64(v));
        rng.push(Fp::from_u64(bit));
        if i < 15 {
            v >>= 1;
        }
    }

    let witness = air.trace(&[cons, rng]);
    (air, witness, publics)
}

pub fn join_split(h: &Poseidon) -> Inner {
    let (air, witness, publics) = join_split_fixture();
    let proof = stark_prove_poseidon_ext_pub(&air, &witness, NQ, GRIND, EXTRA, h, &publics);
    let ci = compose_inputs_pub(&air, &proof, EXTRA, h, &publics);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());
    Inner {
        air,
        publics,
        proof,
        ci,
        t,
        g,
        sidecar: None,
    }
}

/// A zkolang step AIR as the inner proof: compile a small program, run it, prove
/// it Poseidon-committed, and replay its composition inputs. The public io binds
/// as AIR boundaries (not transcript publics), so the non-pub prove and
/// `compose_inputs` are the matching pair, as in the step tests.
pub fn step_air(h: &Poseidon) -> Inner<nonos_zkolang::StepAir> {
    use nonos_zkolang::{compile_source, program_log_t, StepAir, Vm};
    let program = compile_source("input x; let y = x * x; output y;").expect("compile");
    let mut vm = Vm::new();
    let trace = vm.run(&program, &[Fp::from_u64(3)], 1).expect("run");
    // Size the inner exactly as the verifier key does, so the recursion attests
    // the registered program identity rather than a differently-padded twin.
    let log_t = program_log_t(&program).expect("program has a halt within the size cap");
    let air =
        StepAir::compile(&program, log_t, &[Fp::from_u64(3)], &[Fp::from_u64(9)]).expect("air");
    let flat = air.build_trace(&trace).expect("layout");
    let proof = stark_prove_poseidon_ext(&air, &flat, NQ, GRIND, EXTRA, h);
    let ci = compose_inputs(&air, &proof, EXTRA, h);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());
    Inner {
        air,
        publics: Vec::new(),
        proof,
        ci,
        t,
        g,
        sidecar: None,
    }
}

/// The recursion over a real transfer rather than a stand-in.
///
/// The fixture above is two toy regions with synthetic publics, sized so the
/// assembly could be built at all. This is the deployed join-split: depth 32
/// against the pool tree, its intent absorbed as the transcript publics the
/// verifier replays. Anything the recursion says about this one, it says about
/// a transfer somebody could actually send.
pub fn shield_join_split(h: &Poseidon) -> Inner<WiredMultiGen> {
    let js = crate::shield::test::scenario::balanced_deployed(crate::shield::key::Break::None);
    let publics = js.intent.clone();
    let root = periodic_root_poseidon(&js.wired, extra(), h);
    // Memoized within a process: the deployed witness is deterministic, and a
    // diagnose loop that re-proves the same inner every iteration turns
    // minutes of thinking into half-hours of waiting.
    static PRE: std::sync::OnceLock<crate::crypto::stark::air::StarkProofExtPPre> =
        std::sync::OnceLock::new();
    let pre = PRE
        .get_or_init(|| {
            stark_prove_poseidon_pre_pub(&js.wired, &js.witness, NQ, GRIND, extra(), h, &publics)
        })
        .clone();
    let ci = compose_inputs_pre(&js.wired, &pre, extra(), h, &publics);
    let t = 1u64 << js.wired.log_trace_len();
    let g = root_of_unity(js.wired.log_trace_len());
    let sidecar = Some(Sidecar {
        periodic_z: pre.periodic_z,
        openings: pre.openings,
        root,
    });
    Inner {
        air: js.wired,
        publics,
        proof: pre.proof,
        ci,
        t,
        g,
        sidecar,
    }
}
