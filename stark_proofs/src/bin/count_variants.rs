// NONOS Operating System (AGPL-3.0-or-later)
//! How many distinct circuits a pool of spends needs.
//!
//! Two spends that differ only in which notes they spend were found to
//! differ in sixteen periodic columns, which means a spend's circuit depends
//! on its inputs. That alone does not say whether a pool is deployable. What
//! decides it is the count of distinct circuits:
//!
//!   small and bounded   one verifier per variant, the settler declares
//!                       which at open, and a wrong declaration fails the
//!                       walk rather than forging anything
//!   one per leaf index  no deployment exists and the circuit must change
//!
//! So this counts rather than diagnoses. It sweeps leaf positions, hashes
//! each assembly's periodic columns, and reports the distinct set with the
//! positions that produced each.
//!
//! Hashing the columns rather than committing them is exact for this
//! purpose: the periodic root is a function of the columns alone, so two
//! assemblies with identical columns have identical roots, and two with
//! different columns have different roots unless keccak collides. It costs
//! one assembly per point instead of an eighteen minute tree build.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::crypto::stark::hash::keccak256;
use stark_proofs::recursion_assembly::build::assemble_over;
use stark_proofs::recursion_assembly::{inner, Tamper};
use std::collections::BTreeMap;
use std::time::Instant;

fn digest_of(cols: &[Vec<stark_proofs::crypto::stark::field::Fp>]) -> String {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&(cols.len() as u64).to_le_bytes());
    for col in cols {
        buf.extend_from_slice(&(col.len() as u64).to_le_bytes());
        for v in col {
            buf.extend_from_slice(&v.to_u64().to_le_bytes());
        }
    }
    keccak256(&buf)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>()[..16]
        .to_string()
}

fn main() {
    /*
     * Positions chosen so parity, depth of shared prefix and absolute index
     * all vary independently. If the wiring turns on the bottom direction
     * alone, these collapse to four classes by the parities of the pair. If
     * it turns on more of the index, the count grows with the sweep.
     */
    let cases: [(usize, usize, usize); 10] = [
        (0, 1, 4),
        (1, 2, 4),
        (2, 3, 4),
        (0, 3, 4),
        (1, 3, 8),
        (0, 2, 8),
        (4, 5, 8),
        (5, 6, 8),
        (6, 7, 8),
        (2, 5, 8),
    ];

    let h = inner::hasher();
    let mut seen: BTreeMap<String, Vec<(usize, usize)>> = BTreeMap::new();
    for (ia, ib, leaves) in cases {
        let t = Instant::now();
        let js = stark_proofs::shield::live::unshield_at(ia, ib, leaves, 1_000_000_000_000);
        let asm = assemble_over(
            &h,
            inner::shield_join_split_of(&h, js, None),
            Tamper::None,
            usize::MAX,
        );
        let d = digest_of(&asm.wired.periodic_columns());
        println!(
            "leaves {leaves:2}  spend ({ia},{ib})  parity ({},{})  periodic {d}  [{:?}]",
            ia % 2,
            ib % 2,
            t.elapsed()
        );
        seen.entry(d).or_default().push((ia, ib));
    }

    println!(
        "\ndistinct circuits across {} spends: {}",
        cases.len(),
        seen.len()
    );
    for (d, who) in &seen {
        let parities: Vec<(usize, usize)> = who.iter().map(|(a, b)| (a % 2, b % 2)).collect();
        println!("  {d}  from {who:?}  parities {parities:?}");
    }
    println!(
        "\n{}",
        match seen.len() {
            1 => "one circuit serves every spend: deployable with a single verifier",
            2..=8 =>
                "a small bounded set: deployable with one verifier per variant, \
                      declared at open",
            _ =>
                "the count is growing with the sweep: no bounded deployment, the \
                  circuit has to change",
        }
    );
}
