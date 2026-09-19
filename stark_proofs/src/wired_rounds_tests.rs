// NONOS Operating System (AGPL-3.0-or-later)
// The copy constraint argued at a point the prover could not aim at.
//
// The region columns are committed, beta and gamma come out of the transcript
// against that root, and only then are the permutation columns built and
// committed. A full prove and verify over that shape is what these check, plus
// the two ways it could be got wrong: a verifier that draws the challenges
// somewhere else, and a proof that claims the row splits somewhere else.

use crate::crypto::stark::air::{
    stark_prove_ext_rounds, stark_verify_ext_rounds, Air, AirExt, GpGroup, StarkProofExtRounds,
    WiredMultiExt,
};
use crate::crypto::stark::field::{Felt, Fp, Fp2};

const W: usize = 12;
const LOG_T: u32 = 6;
const QUERIES: usize = 16;
const GRIND: u32 = 4;
const BLOWUP: u32 = 0;

/// A region whose every column holds its value, so wiring two cells of one
/// column is satisfiable and wiring across columns is not.
struct Hold {
    seeds: Vec<Fp>,
}

impl Hold {
    fn body<F: Felt>(&self, window: &[F]) -> Vec<F> {
        (0..W).map(|c| window[W + c] - window[c]).collect()
    }

    fn trace(&self) -> Vec<Fp> {
        let rows = 1usize << LOG_T;
        let mut t = vec![Fp::ZERO; rows * W];
        for r in 0..rows {
            for c in 0..W {
                t[r * W + c] = self.seeds[c];
            }
        }
        t
    }
}

impl AirExt for Hold {
    fn transition_ext(&self, window: &[Fp2], _p: &[Fp2]) -> Vec<Fp2> {
        self.body(window)
    }
}

impl Air for Hold {
    fn log_trace_len(&self) -> u32 {
        LOG_T
    }
    fn trace_width(&self) -> usize {
        W
    }
    fn window_size(&self) -> usize {
        2
    }
    fn constraint_degree(&self) -> usize {
        1
    }
    fn num_transition(&self) -> usize {
        W
    }
    fn transition(&self, window: &[Fp], _p: &[Fp]) -> Vec<Fp> {
        self.body(window)
    }
    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        (0..W).map(|c| (c, 0, self.seeds[c])).collect()
    }
}

fn region() -> Hold {
    Hold {
        seeds: (0..W).map(|c| Fp::from_u64(1_000 + c as u64)).collect(),
    }
}

/// Row 0 of every column is bound to row 1 of the same column.
fn sigma(total: usize) -> Vec<usize> {
    let mut s: Vec<usize> = (0..total * W).collect();
    for c in 0..W {
        s[c] = W + c;
        s[W + c] = c;
    }
    s
}

fn air() -> WiredMultiExt {
    let boxed: Vec<Box<dyn AirExt>> = vec![Box::new(region())];
    let total = 1usize << Air::log_trace_len(&region());
    let perm = GpGroup {
        wired_cols: (0..W).collect(),
        sigma: sigma(total * 4),
        beta: Fp::from_u64(5),
        gamma: Fp::from_u64(7),
    };
    WiredMultiExt::new_kinds_chained(boxed, &[0], perm, Vec::new())
}

/// An honest proof in two rounds verifies.
#[test]
fn a_two_round_proof_verifies() {
    let a = air();
    let mut trace = a.trace(&[region().trace()]);
    let (rounds, p_tree, _) =
        stark_prove_ext_rounds(air(), &mut trace, QUERIES, GRIND, BLOWUP, &[], None, &[])
            .expect("the two round prover produces a proof");

    assert_ne!(
        rounds.pre.proof.trace_root, rounds.perm_root,
        "the two rounds committed the same thing"
    );
    assert_eq!(
        rounds.region_width,
        a.region_width(),
        "the emitted split is not where the permutation columns start"
    );
    assert!(stark_verify_ext_rounds(
        air(),
        &rounds,
        QUERIES,
        GRIND,
        BLOWUP,
        &p_tree.root(),
        &[],
    ));
}

/// A verifier that keeps the circuit's own challenges instead of drawing them
/// rejects an honest proof, which is the shape of the bug this replaces: the
/// two sides have to agree that the point came from the transcript.
#[test]
fn the_challenges_are_not_the_circuit_constants() {
    let mut trace = air().trace(&[region().trace()]);
    let (rounds, p_tree, proved) =
        stark_prove_ext_rounds(air(), &mut trace, QUERIES, GRIND, BLOWUP, &[], None, &[])
            .expect("the two round prover produces a proof");

    let (beta, gamma) = proved.challenges();
    assert!(
        beta != Fp::from_u64(5) || gamma != Fp::from_u64(7),
        "the drawn challenges are the baked ones, so this proves nothing"
    );

    // The proof is sound; it is the split that must not be negotiable.
    let mut lied = rounds;
    lied.region_width += 1;
    assert!(!stark_verify_ext_rounds(
        air(),
        &lied,
        QUERIES,
        GRIND,
        BLOWUP,
        &p_tree.root(),
        &[],
    ));
}

/// A trace whose copy constraint does not hold does not produce a verifying
/// proof, whatever the prover does with the permutation columns afterwards.
#[test]
fn a_broken_binding_does_not_verify() {
    let a = air();
    let mut trace = a.trace(&[region().trace()]);
    let stride = Air::trace_width(&a);
    // Row 1 of column 0 is wired to row 0 of column 0. Move it.
    trace[stride] = trace[stride] + Fp::ONE;

    let proved = stark_prove_ext_rounds(air(), &mut trace, QUERIES, GRIND, BLOWUP, &[], None, &[]);
    match proved {
        None => {}
        Some((rounds, p_tree, _)) => assert!(!stark_verify_ext_rounds(
            air(),
            &rounds,
            QUERIES,
            GRIND,
            BLOWUP,
            &p_tree.root(),
            &[],
        )),
    }
}

/// A blinded proof verifies, and it does not hand over the trace.
///
/// A proof opens `QUERIES` rows and a frame of `window_size` more. Unblinded,
/// those cells are the witness, which for the shield is the spend. Blinded,
/// `f + r * (x^t - 1)` agrees with `f` on the trace domain and is random off
/// it, so the constraints still hold where they are checked and the opened
/// rows are not the trace.
///
/// Two proofs of one statement under different blinds are the test. If the
/// openings match, nothing was hidden. If either fails to verify, the hiding
/// broke the argument.
#[test]
fn a_blinded_proof_verifies_and_does_not_open_the_trace() {
    use crate::crypto::stark::air::blinding_poly;
    use crate::recursion_assembly::inner;

    let h = inner::hasher();
    let deg = QUERIES + 2;
    // Every committed column, accumulators included: they are opened too.
    let width = Air::trace_width(&air());
    let blinds = |tag: u64| -> Vec<Vec<Fp>> {
        let seed = [
            Fp::from_u64(tag),
            Fp::from_u64(2),
            Fp::from_u64(3),
            Fp::from_u64(4),
        ];
        (0..width)
            .map(|c| blinding_poly(&h, &seed, c, deg))
            .collect()
    };

    let plain = {
        let mut t = air().trace(&[region().trace()]);
        stark_prove_ext_rounds(air(), &mut t, QUERIES, GRIND, BLOWUP, &[], None, &[])
            .expect("the unblinded prover produces a proof")
    };
    let one = {
        let mut t = air().trace(&[region().trace()]);
        stark_prove_ext_rounds(air(), &mut t, QUERIES, GRIND, BLOWUP, &[], None, &blinds(101))
            .expect("the blinded prover produces a proof")
    };
    let two = {
        let mut t = air().trace(&[region().trace()]);
        stark_prove_ext_rounds(air(), &mut t, QUERIES, GRIND, BLOWUP, &[], None, &blinds(202))
            .expect("the blinded prover produces a proof")
    };

    for (what, p) in [("blind one", &one), ("blind two", &two)] {
        assert!(
            stark_verify_ext_rounds(air(), &p.0, QUERIES, GRIND, BLOWUP, &p.1.root(), &[]),
            "{what} does not verify, so the hiding broke the argument"
        );
    }

    let rows = |p: &StarkProofExtRounds| -> Vec<Vec<Fp>> {
        p.pre.proof.queries.iter().map(|q| q.trace.clone()).collect()
    };
    assert_ne!(
        rows(&one.0),
        rows(&plain.0),
        "a blinded proof opened the same cells as the unblinded one"
    );
    assert_ne!(
        rows(&one.0),
        rows(&two.0),
        "two blinds produced the same openings, so the blind is not doing anything"
    );
}

/// The real outer, chained and proved in two rounds, verifies.
///
/// Capped to a couple of inner queries so it runs in minutes rather than
/// hours, which changes the trace length and nothing about the shape: the same
/// regions, the same wiring families, the same two commitments. This is the
/// gate the settlement emit sits behind, because everything above it is a toy
/// twelve columns wide and the thing that ships is five hundred and fifty six.
#[test]
#[ignore]
fn the_real_outer_proves_and_verifies_in_two_rounds() {
    use crate::recursion_assembly::build::{assemble_over_wired, Wiring};
    use crate::recursion_assembly::{inner, Tamper};
    use crate::shield_params::deployment;

    let h = inner::hasher();
    let inner_js = inner::shield_join_split(&h);
    let mut asm = assemble_over_wired(&h, inner_js, Tamper::None, 2, Wiring::Chained);
    let width = Air::trace_width(&asm.wired);
    let split = asm.wired.region_width();
    std::println!(
        "chained outer: width {width}, split at {split}, degree {}, periodic {}",
        Air::constraint_degree(&asm.wired),
        Air::periodic_columns(&asm.wired).len()
    );

    /*
     * Blinded, because the point of this circuit is that a spend reveals
     * nothing and a proof opens 32 rows of the trace plus a two row frame.
     * One polynomial per column, long enough to cover every cell the proof
     * hands over.
     */
    let deg = deployment::N_QUERIES + Air::window_size(&asm.wired);
    let seed = [Fp::from_u64(11), Fp::from_u64(22), Fp::from_u64(33), Fp::from_u64(44)];
    let blind: Vec<Vec<Fp>> = (0..width)
        .map(|c| crate::crypto::stark::air::blinding_poly(&h, &seed, c, deg))
        .collect();

    let mut witness = core::mem::take(&mut asm.witness);
    let publics = asm.publics.clone();
    let (rounds, p_tree, proved) = stark_prove_ext_rounds(
        asm.wired,
        &mut witness,
        deployment::N_QUERIES,
        deployment::GRIND_BITS,
        deployment::EXTRA_BLOWUP_BITS,
        &publics,
        None,
        &blind,
    )
    .expect("the two round prover produces a proof for the real outer");

    let (beta, gamma) = proved.challenges();
    std::println!(
        "beta {} gamma {} drawn from the region root",
        beta.to_u64(),
        gamma.to_u64()
    );
    assert!(
        beta != Fp::from_u64(5) || gamma != Fp::from_u64(7),
        "the drawn challenges are the circuit constants"
    );
    assert_eq!(rounds.region_width, split);

    assert!(
        stark_verify_ext_rounds(
            proved,
            &rounds,
            deployment::N_QUERIES,
            deployment::GRIND_BITS,
            deployment::EXTRA_BLOWUP_BITS,
            &p_tree.root(),
            &publics,
        ),
        "the real outer's two round proof does not verify"
    );
}
