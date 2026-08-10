// NONOS Operating System (AGPL-3.0-or-later)
//! No-inflation for a 2-in-2-out join-split: the values being spent equal the
//! values being created plus whatever leaves publicly.
//!
//! The property is only worth anything if the values in the sum are the values
//! the notes actually committed to. A circuit that proves `a + b = c + d` over
//! numbers a prover supplies alongside four commitments proves nothing: the
//! prover picks small numbers for the inputs it does not own and large ones for
//! the outputs it does. So every term in the balance is copy-constrained back to
//! the value limbs inside its own note commitment.
//!
//! Layout, one row per term in `ValueBalance`:
//!
//! ```text
//!   row 0  input  note A      row 2  output note C
//!   row 1  input  note B      row 3  output note D
//!   row 4  output public_amount     row 5  output fee
//! ```
//!
//! A note commits its value as `(lo, hi)` in limbs 0 and 1, and those limbs ride
//! the first row of the note circuit's first opening, columns 0 and 1. Binding
//! is therefore two copy constraints per note, straight from the commitment's
//! own preimage into the balance row.
//!
//! `public_amount` and `fee` are public words rather than notes, so they carry
//! no commitment to bind against; the join-split assembly binds them to their
//! public inputs. They are represented here as `(value, 0)`.
//!
//! Out of scope on purpose: ownership and double-spend. Those are the nullifier,
//! which needs the key hierarchy pinned in SPEC.md §4. This region proves value
//! is conserved, not that the spender was entitled to spend it.

use crate::crypto::stark::air::{
    AirExt, GpGroup, Leg, ValueBalance, WiredMultiExt,
};
use crate::crypto::stark::field::Fp;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::note_commit::{note_edges, note_parts, Note, NoteParts, POOL_LOG_ROUNDS};
use crate::note_member::{note_member, PoolTree};

/// Rows in the balance region: four notes plus the two public legs.
const N_TERMS: usize = 6;
const LOG_T: u32 = 3;

pub(crate) struct JoinSplit {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
}

fn equate(span: usize, cols: Vec<usize>, swaps: &[(usize, usize, usize, usize)]) -> GpGroup {
    let k = cols.len();
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for &(ra, ca, rb, cb) in swaps {
        let ia = cols.iter().position(|&c| c == ca).unwrap();
        let ib = cols.iter().position(|&c| c == cb).unwrap();
        sigma.swap(ra * k + ia, rb * k + ib);
    }
    GpGroup { wired_cols: cols, sigma, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
}

/// Split a value the way a note commits it.
fn limbs_of(v: u64) -> (Fp, Fp) {
    (Fp::from_u64(v & 0xFFFF_FFFF), Fp::from_u64(v >> 32))
}

/// Build the join-split conservation circuit. `unbound_value` breaks the tie
/// between the balance and a commitment: the arithmetic still sums to zero, but
/// the first input's balance row no longer carries the value its note committed
/// to. That is the forgery the copy constraints exist to stop, and it is the
/// only thing that may reject.
/// A filler note, so the pool tree has unrelated leaves.
fn decoy(seed: u64) -> Note {
    Note {
        value: seed,
        asset_id: 0,
        spend_pk: [seed, seed + 1, seed + 2, seed + 3],
        blinding: [seed + 4, seed + 5, seed + 6, seed + 7],
    }
}

/// A commitment that IS in the tree but is not the note being spent.
fn tree_decoy_cm(_t: &mut PoolTree) -> [Fp; RATE] {
    note_parts(&decoy(100)).cm
}

pub(crate) fn join_split(
    inputs: [&Note; 2],
    outputs: [&Note; 2],
    public_amount: u64,
    fee: u64,
    unbound_value: bool,
    spend_unproven: bool,
) -> JoinSplit {
    let notes: [&Note; 4] = [inputs[0], inputs[1], outputs[0], outputs[1]];
    let parts: Vec<NoteParts> = notes.iter().map(|n| note_parts(n)).collect();
    let spans: Vec<usize> = parts.iter().map(|p| p.span_op).collect();
    let cms: Vec<[Fp; RATE]> = parts.iter().map(|p| p.cm).collect();

    // The balance terms, in the row order the legs declare.
    let mut terms: Vec<(Fp, Fp)> = Vec::with_capacity(N_TERMS);
    for (i, n) in notes.iter().enumerate() {
        // The tamper claims a value the note did not commit to. The sum is kept
        // consistent below, so only the binding can catch it.
        let v = if unbound_value && i == 0 { n.value.wrapping_add(1) } else { n.value };
        terms.push(limbs_of(v));
    }
    let pa = if unbound_value { public_amount.wrapping_add(1) } else { public_amount };
    terms.push(limbs_of(pa));
    terms.push(limbs_of(fee));

    let legs = alloc::vec![
        Leg::Input,  // input A
        Leg::Input,  // input B
        Leg::Output, // output C
        Leg::Output, // output D
        Leg::Output, // public amount
        Leg::Output, // fee
        Leg::Pad,
        Leg::Pad,
    ];
    let balance = ValueBalance { log_t: LOG_T, legs };
    let btrace = balance.trace(&terms);

    // The two spent notes must be notes the pool actually holds. Without this
    // the circuit conserves value over one pair of notes while proving
    // membership of a different pair, which spends money that was never
    // deposited. The commitments are the join: each input's committed value is
    // the leaf its membership walks from.
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let mut tree = PoolTree::new(h.clone());
    // A few unrelated deposits, so the spent notes are not the degenerate first
    // leaves and the paths carry real siblings.
    for s in 100..103u64 {
        tree.insert(note_parts(&decoy(s)).cm);
    }
    let leaves: Vec<usize> = (0..2).map(|i| tree.insert(cms[i])).collect();

    let mut members = Vec::with_capacity(2);
    for (i, &leaf_idx) in leaves.iter().enumerate() {
        // Break the join: prove membership of a note that is in the tree but is
        // not the note being spent. Every other constraint still holds.
        let claimed = if spend_unproven { tree_decoy_cm(&mut tree) } else { cms[i] };
        let (sibs, dirs) = tree.path(leaf_idx);
        members.push((claimed, sibs, dirs));
    }

    // One flat stack: the balance, a note region per term, then the two
    // membership proofs for the spent notes.
    let mut regions: Vec<Box<dyn AirExt>> = alloc::vec![Box::new(balance)];
    let mut traces: Vec<Vec<Fp>> = alloc::vec![btrace];
    for p in parts {
        regions.push(Box::new(p.region));
        traces.push(p.trace);
    }
    let mut leaf_cols = Vec::with_capacity(2);
    for (leaf, sibs, dirs) in members {
        // The leaf rides the low lanes or the high lanes of the opening's first
        // row depending on the first direction bit, exactly as `inject` places
        // it. Reading the wrong half is the index-parity trap.
        leaf_cols.push(if dirs[0] { RATE } else { 0 });
        let m = note_member(&h, leaf, sibs, dirs, tree.root());
        regions.push(Box::new(m.region));
        traces.push(m.witness);
    }

    let mut offsets = Vec::with_capacity(regions.len());
    let mut row = 0usize;
    for r in &regions {
        offsets.push(row);
        row += 1usize << r.log_trace_len();
    }
    let span = row.next_power_of_two();
    let span_op = spans[0];

    let mut groups: Vec<GpGroup> = Vec::new();
    for i in 0..4 {
        let base = offsets[1 + i];
        // The note's own compress tree stays chained.
        for sw in note_edges(base, span_op) {
            let cols = if sw.1 == sw.3 { alloc::vec![sw.1] } else { alloc::vec![sw.1, sw.3] };
            groups.push(equate(span, cols, &[sw]));
        }
        // And its committed value limbs ARE the balance row's limbs. Without
        // these the sum is over numbers the prover chose, not over the notes.
        let bal = offsets[0] + i;
        groups.push(equate(span, alloc::vec![0, 1], &[(bal, 1, base, 0)]));
        groups.push(equate(span, alloc::vec![1, 2], &[(bal, 2, base, 1)]));
    }

    // The spent notes are the proven notes: each input's commitment IS the leaf
    // its membership walks from, lane by lane.
    let l = 1usize << POOL_LOG_ROUNDS;
    for i in 0..2 {
        let cm_row = offsets[1 + i] + 2 * span_op + l; // the note's final root
        let leaf_row = offsets[5 + i];
        let lc = leaf_cols[i];
        for c in 0..RATE {
            let cols =
                if c == lc + c { alloc::vec![c, lc + c] } else { alloc::vec![c, lc + c] };
            groups.push(equate(span, cols, &[(cm_row, c, leaf_row, lc + c)]));
        }
    }

    let wired = WiredMultiExt::new(regions, groups);
    let witness = wired.trace(&traces);
    JoinSplit { wired, witness }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::stark::air::Air;

    fn note(seed: u64, value: u64) -> Note {
        Note {
            value,
            asset_id: 0,
            spend_pk: [seed + 1, seed + 2, seed + 3, seed + 4],
            blinding: [seed + 5, seed + 6, seed + 7, seed + 8],
        }
    }

    fn satisfies(air: &WiredMultiExt, witness: &[Fp]) -> bool {
        let w = air.trace_width();
        let ws = air.window_size();
        let total = 1usize << air.log_trace_len();
        let periodic = air.periodic_columns();
        for r in 0..total - (ws - 1) {
            let mut window = Vec::with_capacity(ws * w);
            for k in 0..ws {
                window.extend_from_slice(&witness[(r + k) * w..(r + k + 1) * w]);
            }
            let per: Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
            if air.transition(&window, &per).iter().any(|v| *v != Fp::ZERO) {
                return false;
            }
        }
        for (col, row, val) in air.boundary() {
            if witness[row * w + col] != val {
                return false;
            }
        }
        true
    }

    /// 1000 + 2000 spent, 1500 + 1200 created, 200 out publicly, 100 in fees.
    fn balanced() -> ([Note; 2], [Note; 2], u64, u64) {
        ([note(0, 1000), note(10, 2000)], [note(20, 1500), note(30, 1200)], 200, 100)
    }

    #[test]
    fn a_conserving_join_split_satisfies() {
        let (i, o, pa, fee) = balanced();
        let js = join_split([&i[0], &i[1]], [&o[0], &o[1]], pa, fee, false, false);
        assert!(satisfies(&js.wired, &js.witness), "an honest join-split must satisfy");
    }

    /// The forgery the bindings exist to stop: claim a different value than the
    /// note committed to, and keep the sum consistent so the balance constraint
    /// is satisfied. Only the copy constraint to the commitment can catch it.
    #[test]
    fn a_value_unbound_from_its_commitment_rejects() {
        let (i, o, pa, fee) = balanced();
        let js = join_split([&i[0], &i[1]], [&o[0], &o[1]], pa, fee, true, false);
        assert!(!satisfies(&js.wired, &js.witness), "a value not committed to was accepted");
    }

    /// Minting: outputs exceed inputs. Caught by the running total, not by a
    /// binding, so this covers the other half of no-inflation.
    #[test]
    fn creating_value_from_nothing_rejects() {
        let (i, _o, pa, fee) = balanced();
        let fat = [note(20, 1500), note(30, 99_999)];
        let js = join_split([&i[0], &i[1]], [&fat[0], &fat[1]], pa, fee, false, false);
        assert!(!satisfies(&js.wired, &js.witness), "value was minted");
    }

    /// The join between the money and the pool: conserve value perfectly over
    /// two notes while proving membership of a different note that really is in
    /// the tree. Balance holds, every commitment is honest, both memberships
    /// walk to the real root. Only the commitment-to-leaf binding can fire, and
    /// without it this spends money that was never deposited.
    #[test]
    fn spending_notes_that_were_never_deposited_rejects() {
        let (i, o, pa, fee) = balanced();
        let js = join_split([&i[0], &i[1]], [&o[0], &o[1]], pa, fee, false, true);
        assert!(!satisfies(&js.wired, &js.witness), "spent a note that is not in the pool");
    }

    /// Burning is equally a break: an unbalanced batch in either direction must
    /// fail, so the total is pinned at both ends rather than bounded one way.
    #[test]
    fn destroying_value_rejects() {
        let (i, _o, pa, fee) = balanced();
        let thin = [note(20, 1), note(30, 1)];
        let js = join_split([&i[0], &i[1]], [&thin[0], &thin[1]], pa, fee, false, false);
        assert!(!satisfies(&js.wired, &js.witness), "value was destroyed");
    }
}
