// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Verify several Poseidon Merkle openings in one STARK, the heavy half of a FRI
//! query verifier: a query opens two values per layer, all under different
//! roots, and this proves the whole batch at once. Each opening occupies a fixed
//! run of rows and is a Merkle path verification; at an opening boundary the
//! state resets to the next opening's public leaf and first sibling. The leaves
//! are public (a FRI proof reveals its openings), and every root is pinned at
//! its checkpoint.

use super::super::field::{Felt, Fp, Fp2};
use super::poseidon::{Poseidon, RATE, WIDTH};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

/// One opening: a public leaf digest, its committed root, and the sibling path.
#[derive(Clone)]
pub struct Opening {
    pub leaf: [Fp; RATE],
    pub root: [Fp; RATE],
    pub siblings: Vec<[Fp; RATE]>,
    pub directions: Vec<bool>,
}

#[derive(Clone)]
pub struct MultiMembership {
    hasher: Poseidon,
    log_rounds: u32,
    depth: usize,
    openings: Vec<Opening>,
    witness_path: bool,
    /// The S-box split: x2 and x4 witnessed per lane, the round constraint
    /// falling from degree 8 to 4. Opt-in, appended columns, so the default
    /// forms and everything built on their coordinates stay byte-identical.
    split: bool,
    /// Pin the bottom index bit. The path direction of the bottom level lives only
    /// in which half of the initial state the leaf occupies, so a scalar that
    /// hashes the position (a nullifier over a leaf index) is otherwise free to
    /// disagree with it in that one bit and retire the note under the sibling
    /// position. This witnesses the bottom direction as a bit and a canonical leaf
    /// it selects from the two halves, so the assembly can bind the bit to the
    /// recovered scalar. Opt-in, appended columns, so the default forms and the
    /// coordinates built on them stay byte-identical.
    pin0: bool,
}

impl MultiMembership {
    /// Build the AIR for a batch of equal-depth openings under `hasher`. The
    /// siblings, directions, and the roots ride the periodic columns and boundaries:
    /// instance-specific structure, fine for a per-proof AIR.
    pub fn new(hasher: Poseidon, log_rounds: u32, openings: Vec<Opening>) -> MultiMembership {
        let depth = openings.first().map(|o| o.siblings.len()).unwrap_or(0);
        MultiMembership { hasher, log_rounds, depth, openings, witness_path: false, split: false, pin0: false }
    }

    /// The production form: the sibling and direction of each compression ride the
    /// trace (a boolean-constrained direction plus RATE sibling columns), so the AIR
    /// is instance-independent (round constants, the slot and opening selectors, and
    /// the reset column are the only periodic columns; no boundary is pinned). The
    /// opened leaf is bound by the assembly grand product to where the fold consumes
    /// it, and the root at the checkpoint is bound to the transcript-absorbed root,
    /// so the path authenticates the fold value against the committed root.
    pub fn new_witness(
        hasher: Poseidon,
        log_rounds: u32,
        openings: Vec<Opening>,
    ) -> MultiMembership {
        let depth = openings.first().map(|o| o.siblings.len()).unwrap_or(0);
        MultiMembership { hasher, log_rounds, depth, openings, witness_path: true, split: false, pin0: false }
    }

    /// The production form with the S-box split: two witnessed squares per
    /// lane after the sibling columns, ceiling 4 instead of 8. Everything
    /// else is `new_witness` to the cell.
    pub fn new_witness_split(
        hasher: Poseidon,
        log_rounds: u32,
        openings: Vec<Opening>,
    ) -> MultiMembership {
        let depth = openings.first().map(|o| o.siblings.len()).unwrap_or(0);
        MultiMembership { hasher, log_rounds, depth, openings, witness_path: true, split: true, pin0: false }
    }

    /// The production form with the bottom index bit pinned: a witnessed bottom
    /// direction and a canonical leaf selected from the two halves, appended after
    /// the sibling columns. Everything else is `new_witness` to the cell. The
    /// assembly binds the bit column to the recovered index scalar's low bit, and
    /// the fold to the canonical leaf, so the position a nullifier hashes cannot
    /// disagree with the one the path authenticated in its bottom bit.
    pub fn new_witness_pin0(
        hasher: Poseidon,
        log_rounds: u32,
        openings: Vec<Opening>,
    ) -> MultiMembership {
        let depth = openings.first().map(|o| o.siblings.len()).unwrap_or(0);
        MultiMembership { hasher, log_rounds, depth, openings, witness_path: true, split: false, pin0: true }
    }

    /// The column carrying the witnessed bottom direction, when pinned. Appended
    /// after the state, direction, and sibling columns.
    pub fn dir0_col(&self) -> usize {
        WIDTH + 1 + RATE
    }

    /// The first column of the canonical leaf, when pinned: the leaf the fold binds
    /// to, selected from the two initial-state halves by the bottom direction.
    pub fn leaf_col(&self) -> usize {
        self.dir0_col() + 1
    }

    /// The trace cell holding opening `o`'s witnessed bottom direction: the row it
    /// starts on, and the direction column. The assembly binds the recovered
    /// scalar's low bit here.
    pub fn dir0_cell(&self, o: usize) -> (usize, usize) {
        (o * self.span(), self.dir0_col())
    }

    fn rounds(&self) -> usize {
        1usize << self.log_rounds
    }

    /// Slots per opening: a compression per level plus the root checkpoint.
    /// Unpadded. This region is most of the assembly's rows, and rounding here
    /// charged 32 slots for 19 at the depth the pool runs.
    fn slots(&self) -> usize {
        self.depth + 1
    }

    /// Openings padded to a power of two.
    fn log_batch(&self) -> u32 {
        self.openings.len().next_power_of_two().max(1).trailing_zeros()
    }

    /// Rows per opening.
    fn span(&self) -> usize {
        self.slots() * self.rounds()
    }

    /// Rows of real work. Padding now sits after them rather than inside every
    /// opening, so a stack can place these and leave the rest.
    fn work_rows(&self) -> usize {
        (1usize << self.log_batch()) * self.span()
    }

    fn initial_state(&self, opening: &Opening) -> [Fp; WIDTH] {
        inject(opening.leaf, opening.siblings[0], opening.directions[0])
    }

    /// The witness trace for this batch: run each opening's Merkle path, reset to
    /// the next opening's leaf at each opening boundary, and let padding rows run
    /// the permutation. The single source of truth for the layout, so callers
    /// prove against exactly what the constraints check.
    pub fn trace(&self) -> Vec<Fp> {
        let l = self.rounds();
        let span = self.span();
        let depth = self.depth;
        let count = self.openings.len();
        let n = 1usize << self.log_trace_len();
        let w = self.trace_width();

        let mut trace = alloc::vec![Fp::ZERO; n * w];
        let sq_base = WIDTH + 1 + RATE;
        let mut state = self.initial_state(&self.openings[0]);
        for r in 0..n {
            trace[r * w..r * w + WIDTH].copy_from_slice(&state);
            if self.split {
                for j in 0..WIDTH {
                    let x2 = state[j] * state[j];
                    trace[r * w + sq_base + j] = x2;
                    trace[r * w + sq_base + WIDTH + j] = x2 * x2;
                }
            }
            let pr = self.hasher.round_with_rc(&state, &self.hasher.round_constant(r % l));
            let opening = r / span;
            let within = r % span;
            let at_row_bnd = within % l == l - 1;
            let is_op_bnd = within == span - 1 && opening + 1 < count;
            let is_slot_bnd = at_row_bnd && within < depth * l && !is_op_bnd;
            // In the production form the compression's direction and sibling ride the
            // trace, at exactly the rows the transition reads them.
            if self.witness_path {
                let m = (within + 1) / l;
                let (dir, sib) = if is_slot_bnd && opening < count && m < depth {
                    (self.openings[opening].directions[m], self.openings[opening].siblings[m])
                } else {
                    (false, [Fp::ZERO; RATE])
                };
                trace[r * w + WIDTH] = if dir { Fp::ONE } else { Fp::ZERO };
                for (c, s) in sib.iter().enumerate() {
                    trace[r * w + WIDTH + 1 + c] = *s;
                }
            }
            // The bottom direction and the canonical leaf, written on the row each
            // opening starts, where the select constraint reads the two halves.
            if self.pin0 && within == 0 && opening < count {
                let o = &self.openings[opening];
                trace[r * w + self.dir0_col()] = if o.directions[0] { Fp::ONE } else { Fp::ZERO };
                for (j, v) in o.leaf.iter().enumerate() {
                    trace[r * w + self.leaf_col() + j] = *v;
                }
            }
            if is_op_bnd {
                state = self.initial_state(&self.openings[opening + 1]);
            } else if is_slot_bnd {
                let m = (within + 1) / l;
                let mut digest = [Fp::ZERO; RATE];
                digest.copy_from_slice(&pr[..RATE]);
                if opening < count && m < depth {
                    let o = &self.openings[opening];
                    state = inject(digest, o.siblings[m], o.directions[m]);
                } else {
                    state = inject(digest, [Fp::ZERO; RATE], false);
                }
            } else {
                state = pr;
            }
        }
        trace
    }

    /// The `(row, column)` of each opening's committed scalar in the trace, one
    /// per opening. A Poseidon-committed FRI leaf is `[value, 0, 0, 0]`, so the
    /// scalar is lane zero of the leaf, which the first index bit places in the
    /// low half of the initial state (column zero) or the high half (column
    /// `RATE`). A wiring engine binds these cells to where a fold consumes the
    /// opened value, so the fold runs on exactly what the opening committed.
    pub fn opened_cells(&self) -> alloc::vec::Vec<(usize, usize)> {
        let span = self.span();
        // When the bottom bit is pinned, the fold binds the canonical leaf, at a
        // fixed column, rather than whichever half the direction placed it in. The
        // select constraint ties that canonical cell back to the real half, so the
        // fold still runs on what the opening committed.
        if self.pin0 {
            let col = self.leaf_col();
            return self.openings.iter().enumerate().map(|(o, _)| (o * span, col)).collect();
        }
        self.openings
            .iter()
            .enumerate()
            .map(|(o, opening)| {
                let col = if opening.directions[0] { RATE } else { 0 };
                (o * span, col)
            })
            .collect()
    }
}

fn inject(node: [Fp; RATE], sibling: [Fp; RATE], right: bool) -> [Fp; WIDTH] {
    let mut state = [Fp::ZERO; WIDTH];
    if !right {
        state[..RATE].copy_from_slice(&node);
        state[RATE..].copy_from_slice(&sibling);
    } else {
        state[..RATE].copy_from_slice(&sibling);
        state[RATE..].copy_from_slice(&node);
    }
    state
}

impl MultiMembership {
    /// The transition over any field, for a recursive verifier that recomputes
    /// this region's constraints inside its own circuit.
    pub fn transition_gen<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        self.transition_impl(window, periodic)
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let stride = self.trace_width();
        let mut state = [F::ZERO; WIDTH];
        state.copy_from_slice(&window[..WIDTH]);
        let mut rc = [F::ZERO; WIDTH];
        rc.copy_from_slice(&periodic[..WIDTH]);
        let slot_bnd = periodic[WIDTH];
        let op_bnd = periodic[WIDTH + 1];
        // Direction and sibling ride the periodic columns in the per-proof form and
        // the trace in the production form. Only the per-proof form carries the next
        // opening's initial state as a periodic reset: that state is built from the
        // opening's own leaf and sibling, so in the production form it would put
        // witness into the columns a verifier key binds, and two transfers would
        // need two keys.
        let mut sib = [F::ZERO; RATE];
        let mut reset = [F::ZERO; WIDTH];
        let dir;
        if self.witness_path {
            dir = window[WIDTH];
            sib.copy_from_slice(&window[WIDTH + 1..WIDTH + 1 + RATE]);
        } else {
            dir = periodic[WIDTH + 2];
            sib.copy_from_slice(&periodic[WIDTH + 3..WIDTH + 3 + RATE]);
            reset.copy_from_slice(&periodic[WIDTH + 3 + RATE..WIDTH + 3 + RATE + WIDTH]);
        }

        let one = F::ONE;
        let mut squares: Vec<F> = Vec::new();
        let pr = if self.split {
            let sq = WIDTH + 1 + RATE;
            let mut x2 = [F::ZERO; WIDTH];
            let mut x4 = [F::ZERO; WIDTH];
            x2.copy_from_slice(&window[sq..sq + WIDTH]);
            x4.copy_from_slice(&window[sq + WIDTH..sq + 2 * WIDTH]);
            let (pr, c2, c4) = self.hasher.round_split_generic(&state, &x2, &x4, &rc);
            squares.extend(c2);
            squares.extend(c4);
            pr
        } else {
            self.hasher.round_generic(&state, &rc)
        };

        let mut out = Vec::with_capacity(WIDTH + 1 + squares.len());
        for (j, next) in window[stride..stride + WIDTH].iter().enumerate() {
            let slot_inject = if j < RATE {
                (one - dir) * pr[j] + dir * sib[j]
            } else {
                (one - dir) * sib[j - RATE] + dir * pr[j - RATE]
            };
            // At an opening boundary the production form leaves the next state to
            // the witness. It is not free: the caller binds the opened leaf to what
            // the opening authenticates and the walked root to the committed root,
            // so a chosen initial state has to be a real path to a bound leaf.
            let carry = if self.witness_path { op_bnd * *next } else { op_bnd * reset[j] };
            let expected = carry + slot_bnd * slot_inject + (one - op_bnd - slot_bnd) * pr[j];
            out.push(*next - expected);
        }
        // The witnessed direction must be a bit, so it cannot blend the two children.
        if self.witness_path {
            out.push(dir * (one - dir));
        }
        out.extend(squares);
        // The bottom direction pin, on the row each opening starts. `d0` is that
        // direction as a bit, and the canonical leaf is the half it selects from
        // the initial state. The fold binds the canonical leaf, so it holds the
        // real leaf only when `d0` names the half the leaf actually occupies,
        // which the walked root already pins to the true position; the assembly
        // then binds `d0` to the recovered scalar's low bit, closing the one bit
        // the path directions left free.
        if self.pin0 {
            let op_start = periodic[WIDTH + 2];
            let d0 = window[self.dir0_col()];
            out.push(op_start * d0 * (one - d0));
            for j in 0..RATE {
                let leaf_c = window[self.leaf_col() + j];
                let selected = (one - d0) * state[j] + d0 * state[RATE + j];
                out.push(op_start * (leaf_c - selected));
            }
        }
        out
    }
}

impl AirExt for MultiMembership {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for MultiMembership {
    fn log_trace_len(&self) -> u32 {
        self.work_rows().next_power_of_two().trailing_zeros()
    }

    fn rows(&self) -> usize {
        self.work_rows()
    }

    fn trace_width(&self) -> usize {
        let base = if self.witness_path { WIDTH + 1 + RATE } else { WIDTH };
        base + if self.split { 2 * WIDTH } else { 0 } + if self.pin0 { 1 + RATE } else { 0 }
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        if self.split {
            4
        } else {
            8
        }
    }

    fn num_transition(&self) -> usize {
        let base = if self.witness_path { WIDTH + 1 } else { WIDTH };
        base + if self.split { 2 * WIDTH } else { 0 } + if self.pin0 { 1 + RATE } else { 0 }
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let l = self.rounds();
        let span = self.span();
        let depth = self.depth;
        let n = 1usize << self.log_trace_len();
        let count = self.openings.len();

        // Per-proof: rc[WIDTH], slot_bnd, op_bnd, dir, sib[RATE], reset[WIDTH].
        // Production: rc[WIDTH], slot_bnd, op_bnd. Dir and sib are trace, and the
        // reset is gone: it held the next opening's leaf and sibling, which is
        // witness, and witness in these columns moves the verifier key per proof.
        let cols_len = if self.witness_path {
            WIDTH + 2 + if self.pin0 { 1 } else { 0 }
        } else {
            WIDTH + 3 + RATE + WIDTH
        };
        let mut cols: Vec<Vec<Fp>> = (0..cols_len).map(|_| Vec::with_capacity(n)).collect();

        for r in 0..n {
            let rc = self.hasher.round_constant(r % l);
            for (j, col) in cols.iter_mut().take(WIDTH).enumerate() {
                col.push(rc[j]);
            }

            let opening = r / span; // which opening this row belongs to
            let within = r % span; // row inside the opening
            let at_row_boundary = within % l == l - 1;
            let is_op_boundary = within == span - 1 && opening + 1 < count;
            // A slot boundary that injects a sibling: last round of a real
            // compression, not the opening's final reset.
            let is_slot_boundary = at_row_boundary && within < depth * l;

            cols[WIDTH].push(if is_slot_boundary && !is_op_boundary { Fp::ONE } else { Fp::ZERO });
            cols[WIDTH + 1].push(if is_op_boundary { Fp::ONE } else { Fp::ZERO });

            // The opening-start selector, one on the first row of each real
            // opening, where the bottom-bit and canonical-leaf constraints apply.
            if self.pin0 {
                cols[WIDTH + 2].push(if within == 0 && opening < count { Fp::ONE } else { Fp::ZERO });
            }

            if !self.witness_path {
                // Reset state to the next opening's initial, at an opening boundary
                // (structurally zero for a single opening).
                let reset = if is_op_boundary && opening + 1 < count {
                    self.initial_state(&self.openings[opening + 1])
                } else {
                    [Fp::ZERO; WIDTH]
                };
                // Sibling and direction for the slot injection at `within`.
                let m = (within + 1) / l;
                let (dir, sib) =
                    if is_slot_boundary && !is_op_boundary && opening < count && m < depth {
                        (self.openings[opening].directions[m], self.openings[opening].siblings[m])
                    } else {
                        (false, [Fp::ZERO; RATE])
                    };
                cols[WIDTH + 2].push(if dir { Fp::ONE } else { Fp::ZERO });
                for (c, s) in sib.iter().enumerate() {
                    cols[WIDTH + 3 + c].push(*s);
                }
                for (c, v) in reset.iter().enumerate() {
                    cols[WIDTH + 3 + RATE + c].push(*v);
                }
            }
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let l = self.rounds();
        let span = self.span();
        let depth = self.depth;

        // Production form: nothing is pinned. The opened leaf is bound by the
        // assembly to the fold, and the root at the checkpoint is bound to the
        // transcript-absorbed root, so both are witness rather than public.
        if self.witness_path {
            return Vec::new();
        }

        let mut b = Vec::with_capacity(WIDTH + self.openings.len() * RATE);
        // The first opening's whole initial state is public.
        let first = self.initial_state(&self.openings[0]);
        for (j, v) in first.iter().enumerate() {
            b.push((j, 0, *v));
        }
        // Every opening's root sits at its checkpoint.
        for (o, opening) in self.openings.iter().enumerate() {
            let row = o * span + depth * l;
            for (c, r) in opening.root.iter().enumerate() {
                b.push((c, row, *r));
            }
        }
        b
    }
}
