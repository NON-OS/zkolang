// NONOS Operating System (AGPL-3.0-or-later)
// The copy constraint argued at a point the prover could not aim at.
//
// The region columns are committed, beta and gamma come out of the transcript
// against that root, and only then are the permutation columns built and
// committed. A full prove and verify over that shape is what these check, plus
// the two ways it could be got wrong: a verifier that draws the challenges
// somewhere else, and a proof that claims the row splits somewhere else.

use crate::crypto::stark::air::{
    stark_prove_ext_rounds, stark_verify_ext_rounds, Air, AirExt, GpGroup, WiredMultiExt,
};
use crate::crypto::stark::field::{Felt, Fp, Fp2};

const W: usize = 12;
const LOG_T: u32 = 3;
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
        stark_prove_ext_rounds(air(), &mut trace, QUERIES, GRIND, BLOWUP, &[], None)
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
        stark_prove_ext_rounds(air(), &mut trace, QUERIES, GRIND, BLOWUP, &[], None)
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

    let proved = stark_prove_ext_rounds(air(), &mut trace, QUERIES, GRIND, BLOWUP, &[], None);
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
