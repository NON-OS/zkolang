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

//! The money-grade wired engine with the copy constraint split across several
//! grand-product columns. Routing every shared value through one permutation makes
//! that constraint's degree the number of wired columns, which for a large assembly
//! blows up the evaluation domain and the on-chain composition check. Splitting the
//! bindings into independent groups, one grand-product column each, keeps every
//! constraint at the size of its group plus a constant, so the AIR degree stays at
//! the region maximum. Same soundness, same bindings, cheap to verify. Layout,
//! region transitions, and each group's product come from `fusion`.

use super::super::field::{Felt, Fp, Fp2};
use super::chained_product;
use super::fusion::{self, Stack};
use super::spec::{Air, AirExt};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// One copy-constraint group: the columns it binds, the permutation over their
/// cells, and its challenges. Each group is an independent grand product.
pub struct GpGroup {
    pub wired_cols: Vec<usize>,
    pub sigma: Vec<usize>,
    pub beta: Fp,
    pub gamma: Fp,
}

pub struct WiredMultiExt {
    regions: Vec<Box<dyn AirExt>>,
    stack: Stack,
    groups: Vec<GpGroup>,
    /*
     * Argue the single group in `groups` as one product chained through
     * intermediate accumulators, instead of one product per group. Packed
     * spends a sigma column per column per group, chained one per column: on
     * the settlement outer, 2,025 against 368. Same permutation either way, so
     * this is cost only, and the packed path is left alone so a circuit that
     * has not moved emits what it emitted.
     */
    chained: bool,
    /// The identity column `r * k + j` depends only on a group's width, and the
    /// product selector is the same column for every group, so both are emitted
    /// once and shared. Only sigma is per group.
    row_idx: usize,
    sig_base: Vec<usize>,
    sel_idx: usize,
    region_transitions: usize,
    /// Assembly-injected constant pins: cells the statement fixes that no
    /// region owns, like a baked commitment root a sidecar authenticates
    /// against. The witness-form regions deliberately pin nothing, so
    /// constants that anchor them enter here.
    extra_boundary: Vec<(usize, usize, Fp)>,
}

impl WiredMultiExt {
    /// Width of each running product. The widest sets the degree, which sets the
    /// evaluation domain.
    /// The shared selector, row-identity and per-group sigma column indices,
    /// for a verifier that reads the permutation from the committed periodic
    /// columns instead of hand-deriving it.
    pub fn permutation_columns(&self) -> (usize, usize, Vec<usize>) {
        (self.sel_idx, self.row_idx, self.sig_base.clone())
    }

    /// Each group's wired columns and challenges, the constraint-side half of
    /// what `permutation_columns` locates.
    pub fn group_params(&self) -> Vec<(Vec<usize>, Fp, Fp)> {
        self.groups
            .iter()
            .map(|g| (g.wired_cols.clone(), g.beta, g.gamma))
            .collect()
    }

    pub fn group_widths(&self) -> Vec<usize> {
        self.groups.iter().map(|g| g.wired_cols.len()).collect()
    }

    /// Set the point the copy constraint is argued at.
    ///
    /// These belong to the proof, not the circuit: a grand product only argues
    /// anything when the prover could not have built its trace against the
    /// point. They are circuit constants today, which is what this is for. The
    /// prover calls it between committing the region columns and building the
    /// permutation columns, and until it does, the assembled default is a
    /// value the prover knows in advance.
    pub fn set_challenges(&mut self, beta: Fp, gamma: Fp) {
        for group in self.groups.iter_mut() {
            group.beta = beta;
            group.gamma = gamma;
        }
    }

    /// The challenges in force, for a verifier that has to agree about them.
    pub fn challenges(&self) -> (Fp, Fp) {
        self.groups
            .first()
            .map(|g| (g.beta, g.gamma))
            .unwrap_or((Fp::ZERO, Fp::ZERO))
    }

    /// Where the permutation columns begin, which is where the two commitment
    /// rounds split: regions below, whatever the challenges produce above.
    pub fn region_width(&self) -> usize {
        self.stack.width
    }

    /// Per region degree. The AIR takes the larger of this and the widest product,
    /// so moving one alone moves nothing.
    pub fn region_degrees(&self) -> Vec<usize> {
        self.regions.iter().map(|r| r.constraint_degree()).collect()
    }

    /*
     * Per kind, in kind order: where the kind's periodic values begin, how many
     * it owns, how many constraint indices its body writes, how many regions
     * run it, and how wide one of those regions is. A verifier evaluating the transition at z needs the first
     * two to slice the periodic vector and the third to know how far into the
     * shared constraint vector that kind reaches; none of the three is
     * recoverable from the trace, and the widest arity here is the overlap
     * width, so a reader that has this does not have to be told it separately.
     * The width is here because a consumer recomputing a kind's transition has
     * to slice the window before it can evaluate anything.
     */
    pub fn kind_map(&self) -> Vec<(usize, usize, usize, usize, usize)> {
        let sel = self.stack.n_kinds;
        (0..self.stack.n_kinds)
            .map(|k| {
                let first = self.stack.kind_first[k];
                let instances = self.stack.kind_of.iter().filter(|&&j| j == k).count();
                (
                    sel + self.stack.kind_slot[k],
                    self.stack.kind_slots[k],
                    self.regions[first].num_transition(),
                    instances,
                    self.regions[first].trace_width(),
                )
            })
            .collect()
    }

    pub fn new(regions: Vec<Box<dyn AirExt>>, groups: Vec<GpGroup>) -> WiredMultiExt {
        let kinds: Vec<usize> = (0..regions.len()).collect();
        WiredMultiExt::new_kinds(regions, &kinds, groups)
    }

    /// `kinds[i]` names region `i`'s kind. Instances of one kind must run equal
    /// constraints over an equal periodic pattern; they then share one selector
    /// and one set of columns instead of carrying an identical copy each.
    pub fn new_kinds(
        regions: Vec<Box<dyn AirExt>>,
        kinds: &[usize],
        groups: Vec<GpGroup>,
    ) -> WiredMultiExt {
        WiredMultiExt::new_kinds_bounded(regions, kinds, groups, Vec::new())
    }

    /// `new_kinds` with constant pins the statement adds on top of the
    /// regions' own boundaries.
    pub fn new_kinds_bounded(
        regions: Vec<Box<dyn AirExt>>,
        kinds: &[usize],
        groups: Vec<GpGroup>,
        extra_boundary: Vec<(usize, usize, Fp)>,
    ) -> WiredMultiExt {
        WiredMultiExt::build(regions, kinds, groups, extra_boundary, false)
    }

    /// The same assembly with the wiring argued as one chained permutation.
    ///
    /// `perm` comes from `recursion_assembly::groups::single`, which has
    /// already been checked to have the declared classes as its cycles. It
    /// rides in the group carrier because that is what it is: one product, one
    /// sigma run, one pair of challenges. Only the accumulator count and the
    /// way the lanes chain differ.
    pub fn new_kinds_chained(
        regions: Vec<Box<dyn AirExt>>,
        kinds: &[usize],
        perm: GpGroup,
        extra_boundary: Vec<(usize, usize, Fp)>,
    ) -> WiredMultiExt {
        WiredMultiExt::build(regions, kinds, alloc::vec![perm], extra_boundary, true)
    }

    fn build(
        regions: Vec<Box<dyn AirExt>>,
        kinds: &[usize],
        groups: Vec<GpGroup>,
        extra_boundary: Vec<(usize, usize, Fp)>,
        chained: bool,
    ) -> WiredMultiExt {
        assert!(
            !chained || groups.len() == 1,
            "the chained argument is one permutation over every wired column, \
             so it takes exactly one group and was given {}",
            groups.len()
        );
        let stack = Stack::of_kinds(&regions, kinds);
        // A kind runs one instance's constraints over every instance's rows, so
        // instances that are not the same AIR swap one region's rules for
        // another's. The caller declares kinds, so check the caller.
        for (i, &k) in kinds.iter().enumerate() {
            let rep = stack.kind_first[k];
            assert!(
                regions[i].trace_width() == regions[rep].trace_width()
                    && regions[i].window_size() == regions[rep].window_size()
                    && regions[i].log_trace_len() == regions[rep].log_trace_len()
                    && regions[i].num_transition() == regions[rep].num_transition()
                    && regions[i].constraint_degree() == regions[rep].constraint_degree()
                    && regions[i].periodic_columns() == regions[rep].periodic_columns(),
                "region {i} is declared kind {k} but does not match instance {rep}"
            );
        }
        let region_slots = stack.kind_slot.last().copied().unwrap_or(0)
            + stack
                .kind_first
                .last()
                .map(|&i| regions[i].periodic_columns().len())
                .unwrap_or(0);
        let base = stack.n_kinds + region_slots;
        let sel_idx = base;
        // The identity a cell is compared against is r * k + j, which is linear in
        // the row, so one column of r serves every group and lane. It used to be a
        // column per lane per distinct width: 21 columns on the recursion, each the
        // full trace length, to carry what multiply and add already give.
        let row_idx = base + 1;
        let mut s = base + 2;
        let mut sig_base = Vec::with_capacity(groups.len());
        for grp in &groups {
            sig_base.push(s);
            s += grp.wired_cols.len();
        }
        for (g, grp) in groups.iter().enumerate() {
            let k = grp.wired_cols.len();
            stack.assert_bound_below_close(&grp.sigma, k, &alloc::format!("group {g}"));
        }
        let mut region_transitions = 0usize;
        for region in &regions {
            region_transitions = region_transitions.max(region.num_transition());
        }
        WiredMultiExt {
            regions,
            stack,
            groups,
            row_idx,
            sig_base,
            sel_idx,
            region_transitions,
            extra_boundary,
            chained,
        }
    }

    /// How many running-product columns the argument needs. The packed form
    /// keeps one per group; the chained form keeps one per accumulator step,
    /// with the last holding the running product itself.
    fn product_columns(&self) -> usize {
        if self.chained {
            chained_product::blocks(self.groups[0].wired_cols.len())
        } else {
            self.groups.len()
        }
    }

    fn stride(&self) -> usize {
        self.stack.width + self.product_columns()
    }

    fn closes_at(&self) -> usize {
        self.stack.closes_at()
    }

    fn ratio(&self, group: &GpGroup, row: &[Fp], r: usize) -> Fp {
        let k = group.wired_cols.len();
        let (b, gm) = (group.beta, group.gamma);
        let mut num = Fp::ONE;
        let mut den = Fp::ONE;
        for (j, &col) in group.wired_cols.iter().enumerate() {
            let v = row[col];
            let id = r * k + j;
            num = num * (v + b * Fp::from_u64(id as u64) + gm);
            den = den * (v + b * Fp::from_u64(group.sigma[id] as u64) + gm);
        }
        num * den.inv()
    }

    /// The witness: regions in the low columns, each group's running product in one
    /// column above them.
    pub fn trace(&self, traces: &[Vec<Fp>]) -> Vec<Fp> {
        let stride = self.stride();
        let total = 1usize << self.log_trace_len();
        let mut trace = fusion::place_traces(&self.stack, &self.regions, stride, total, traces);
        let span = self.closes_at();
        if self.chained {
            self.fill_chained(&mut trace, stride, total, span);
            return trace;
        }
        for (g, group) in self.groups.iter().enumerate() {
            let z_col = self.stack.width + g;
            let mut z = Fp::ONE;
            for r in 0..total {
                trace[r * stride + z_col] = z;
                if r < span {
                    let base = r * stride;
                    z = z * self.ratio(group, &trace[base..base + stride], r);
                }
            }
        }
        trace
    }

    /// The accumulator columns of the chained argument. The last is the
    /// running product, which the boundary pins and the previous row hands on;
    /// the rest are this row's partials, so no lane sees more than `BLOCK`
    /// factors. Off the argument's rows the product holds and the partials are
    /// left where they lie, which nothing reads.
    fn fill_chained(&self, trace: &mut [Fp], stride: usize, total: usize, span: usize) {
        let group = &self.groups[0];
        let k = group.wired_cols.len();
        let blocks = chained_product::blocks(k);
        let (b, gm) = (group.beta, group.gamma);
        let zcol = self.stack.width + blocks - 1;
        let mut z = Fp::ONE;
        for r in 0..total {
            trace[r * stride + zcol] = z;
            if r >= span {
                continue;
            }
            let base = r * stride;
            let mut running = z;
            for m in 0..blocks {
                let lo = m * chained_product::BLOCK;
                let hi = core::cmp::min(lo + chained_product::BLOCK, k);
                let mut num = Fp::ONE;
                let mut den = Fp::ONE;
                for j in lo..hi {
                    let v = trace[base + group.wired_cols[j]];
                    let id = r * k + j;
                    num = num * (v + b * Fp::from_u64(id as u64) + gm);
                    den = den * (v + b * Fp::from_u64(group.sigma[id] as u64) + gm);
                }
                running = running * num * den.inv();
                if m + 1 < blocks {
                    trace[base + self.stack.width + m] = running;
                }
            }
            z = running;
        }
    }

    /// Recompute the running products over a witness whose cells have moved,
    /// leaving the regions' columns alone. What a prover does once it has
    /// decided what its trace says.
    pub fn refill_products(&self, trace: &mut [Fp]) {
        let stride = self.stride();
        let total = trace.len() / stride;
        let span = self.closes_at();
        if self.chained {
            self.fill_chained(trace, stride, total, span);
            return;
        }
        for (g, group) in self.groups.iter().enumerate() {
            let z_col = self.stack.width + g;
            let mut z = Fp::ONE;
            for r in 0..total {
                trace[r * stride + z_col] = z;
                if r < span {
                    let base = r * stride;
                    z = z * self.ratio(group, &trace[base..base + stride], r);
                }
            }
        }
    }

    /// The columns the wiring touches, for a caller asking what the copy
    /// constraint is responsible for holding.
    pub fn wired_columns(&self) -> Vec<usize> {
        let mut c: Vec<usize> = self
            .groups
            .iter()
            .flat_map(|g| g.wired_cols.iter().copied())
            .collect();
        c.sort_unstable();
        c.dedup();
        c
    }

    fn group_product<F: Felt>(&self, g: usize, group: &GpGroup, window: &[F], periodic: &[F]) -> F {
        let b = F::from_base(group.beta);
        let gm = F::from_base(group.gamma);
        let stride = self.stride();
        let width = self.stack.width;
        let z = window[width + g];
        let z_next = window[stride + width + g];
        let sgb = self.sig_base[g];
        let kf = F::from_base(Fp::from_u64(group.wired_cols.len() as u64));
        let row = periodic[self.row_idx];
        let mut num = F::ONE;
        let mut den = F::ONE;
        for (j, &col) in group.wired_cols.iter().enumerate() {
            let v = window[col];
            let id = row * kf + F::from_base(Fp::from_u64(j as u64));
            let sig = periodic[sgb + j];
            num = num * (v + b * id + gm);
            den = den * (v + b * sig + gm);
        }
        let gp_sel = periodic[self.sel_idx];
        let product = z_next * den - z * num;
        let carry = z_next - z;
        gp_sel * product + (F::ONE - gp_sel) * carry
    }

    /// The full transition over any field, with the caller supplying each
    /// region's constraint evaluation. This is how a recursive verifier
    /// recomputes the whole wired AIR inside its own circuit: the layout,
    /// selectors and grand products replay here, and the closure dispatches to
    /// each region's own generic transition, which the boxed form cannot carry.
    pub fn transition_generic<F: Felt>(
        &self,
        window: &[F],
        periodic: &[F],
        region: impl Fn(usize, &[F], &[F]) -> Vec<F>,
    ) -> Vec<F> {
        let mut out = fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stride(),
            window,
            periodic,
            region,
        );
        self.append_groups(&mut out, window, periodic);
        out
    }

    fn append_groups<F: Felt>(&self, out: &mut [F], window: &[F], periodic: &[F]) {
        if self.chained {
            for (m, lane) in self.chained_lanes(window, periodic).into_iter().enumerate() {
                out[self.region_transitions + m] = lane;
            }
            return;
        }
        for (g, group) in self.groups.iter().enumerate() {
            out[self.region_transitions + g] = self.group_product(g, group, window, periodic);
        }
    }

    /// The chained lanes at one window. Slots are numbered `row * k + j` over
    /// the whole permutation rather than per block, so the row column serves
    /// every block just as it serves every group.
    fn chained_lanes<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let group = &self.groups[0];
        let k = group.wired_cols.len();
        let blocks = chained_product::blocks(k);
        let stride = self.stride();
        let width = self.stack.width;
        let sgb = self.sig_base[0];
        let kf = F::from_base(Fp::from_u64(k as u64));
        let row = periodic[self.row_idx];

        let mut steps = Vec::with_capacity(blocks);
        for m in 0..blocks {
            let lo = m * chained_product::BLOCK;
            let hi = core::cmp::min(lo + chained_product::BLOCK, k);
            let values: Vec<F> = (lo..hi).map(|j| window[group.wired_cols[j]]).collect();
            let identity: Vec<F> = (lo..hi)
                .map(|j| row * kf + F::from_base(Fp::from_u64(j as u64)))
                .collect();
            let sigma: Vec<F> = (lo..hi).map(|j| periodic[sgb + j]).collect();
            steps.push(chained_product::step(
                &values,
                &identity,
                &sigma,
                group.beta,
                group.gamma,
            ));
        }

        let acc: Vec<F> = (0..blocks).map(|m| window[width + m]).collect();
        let acc_next: Vec<F> = (0..blocks).map(|m| window[stride + width + m]).collect();
        chained_product::lanes(&steps, &acc, &acc_next, periodic[self.sel_idx])
    }
}

impl AirExt for WiredMultiExt {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        let mut out = fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stride(),
            window,
            periodic,
            |i, l, p| self.regions[i].transition_ext(l, p),
        );
        self.append_groups(&mut out, window, periodic);
        out
    }
}

impl Air for WiredMultiExt {
    fn log_trace_len(&self) -> u32 {
        self.stack.log_trace_len()
    }

    fn trace_width(&self) -> usize {
        self.stride()
    }

    fn window_size(&self) -> usize {
        self.stack.window
    }

    fn constraint_degree(&self) -> usize {
        let mut d = 1usize;
        for region in &self.regions {
            d = d.max(region.constraint_degree());
        }
        /*
         * A lane costs its factors plus the selector, so width plus two. The
         * chained form caps the width at BLOCK however wide the permutation
         * gets, which is why the groups could go.
         */
        let lane_width = if self.chained {
            core::cmp::min(chained_product::BLOCK, self.groups[0].wired_cols.len())
        } else {
            self.groups
                .iter()
                .map(|g| g.wired_cols.len())
                .max()
                .unwrap_or(0)
        };
        (d + 2).max(lane_width + 2)
    }

    fn num_transition(&self) -> usize {
        self.region_transitions + self.product_columns()
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let total = 1usize << self.log_trace_len();
        let span = self.closes_at();
        let mut cols = fusion::base_periodic(&self.stack, &self.regions, total);
        let mut gp_sel = alloc::vec![Fp::ZERO; total];
        for item in gp_sel.iter_mut().take(span) {
            *item = Fp::ONE;
        }
        cols.push(gp_sel);
        let mut row = alloc::vec![Fp::ZERO; total];
        for (r, slot) in row.iter_mut().enumerate().take(span) {
            *slot = Fp::from_u64(r as u64);
        }
        cols.push(row);
        for group in &self.groups {
            let k = group.wired_cols.len();
            for j in 0..k {
                let mut sig = alloc::vec![Fp::ZERO; total];
                for (r, slot) in sig.iter_mut().enumerate().take(span) {
                    *slot = Fp::from_u64(group.sigma[r * k + j] as u64);
                }
                cols.push(sig);
            }
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let mut out = fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stride(),
            window,
            periodic,
            |i, l, p| self.regions[i].transition(l, p),
        );
        self.append_groups(&mut out, window, periodic);
        out
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let mut b = fusion::base_boundary(&self.stack, &self.regions);
        b.extend(self.extra_boundary.iter().copied());
        let span = self.closes_at();
        /*
         * A boundary only says something about a column that crosses rows.
         * Every packed product column does; of the chained ones only the last.
         * The partials are computed by their own lanes from that row's cells,
         * so pinning them would pin a result rather than a claim.
         */
        let pinned = if self.chained {
            self.product_columns() - 1..self.product_columns()
        } else {
            0..self.groups.len()
        };
        for g in pinned {
            b.push((self.stack.width + g, 0, Fp::ONE));
            b.push((self.stack.width + g, span, Fp::ONE));
        }
        b
    }
}
