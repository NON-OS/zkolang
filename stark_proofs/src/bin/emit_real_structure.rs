// NONOS Operating System (AGPL-3.0-or-later)
//! The real outer's shape, from the full query assembly: the structure a
//! settlement verifier derives its constants from, the baked periodic root it
//! holds, and a satisfaction walk over the whole witness so the shape shipped is
//! a shape that provably accepts. No proving here; the proof artifact is the
//! emitter's job. This is the contract side's re-gate input for the real
//! circuit, produced whole or not at all.
//!
//! By default the outer is assembled over the settlement inner, 32 queries at
//! rate 1/16, the point the registered keys hold. With `transfer` it is assembled
//! over a transfer inner at rate 1/4, 56 queries unless `queries=N` says
//! otherwise, which is what the outer would have to verify if senders proved at
//! that point. Run at 56 and at 64 and the emitted layout, rather than an
//! estimate, says what adopting the transfer point costs the recursion and
//! whether the 144 bit count still fits the same trace budget: the outer's span
//! and log trace length are the numbers to read.
//!
//! The outer's authentication regions size the inner's evaluation domain from
//! `inner::extra()`, which the environment sets. A transfer assembly is therefore
//! only consistent under `NONOS_INNER_EXTRA=1`, and the emit refuses to run the
//! transfer mode under any other value rather than emit a shape that does not
//! match the proof it was built over.

use stark_proofs::crypto::stark::air::{Air, COSET_SHIFT};
use stark_proofs::recursion_assembly::build::assemble_over;
use stark_proofs::recursion_assembly::{assemble_real, inner, Tamper};
use stark_proofs::shield_params::{deployment, transfer};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let at_transfer = args.iter().any(|a| a == "transfer");
    /*
     * The query count for a transfer assembly. The transfer point's own count is
     * the default; `queries=N` overrides it so the outer can be assembled at more
     * than one count on the same rate and grind, which is how "does 64 still fit
     * the 2^21 budget" becomes a measurement instead of a model.
     */
    let queries = args
        .iter()
        .find_map(|a| a.strip_prefix("queries=").and_then(|v| v.parse::<usize>().ok()))
        .unwrap_or(transfer::N_QUERIES);
    let out = args
        .iter()
        .find(|a| a.as_str() != "transfer" && !a.starts_with("queries="))
        .cloned()
        .unwrap_or_else(|| {
            if at_transfer {
                format!("transfer-{queries}-structure.json")
            } else {
                "real-structure.json".into()
            }
        });
    let point = if at_transfer { "transfer" } else { "settlement" };

    /*
     * The blowup the outer's authentication assumes for the inner comes from the
     * environment. A transfer inner is proved at rate 1/4, so the environment
     * must say so, or the outer would authenticate a 4x domain as a 16x one and
     * the shape emitted would not be the shape of any proof that verifies.
     */
    if at_transfer && inner::extra() != transfer::EXTRA_BLOWUP_BITS {
        eprintln!(
            "the transfer assembly needs NONOS_INNER_EXTRA={} (found {}); refusing to emit \
             a shape that does not match its inner",
            transfer::EXTRA_BLOWUP_BITS,
            inner::extra()
        );
        std::process::exit(2);
    }

    let h = inner::hasher();
    let t0 = Instant::now();
    let mut asm = if at_transfer {
        let inner_at = inner::shield_join_split_at(
            &h,
            queries,
            transfer::GRIND_BITS,
            transfer::EXTRA_BLOWUP_BITS,
        );
        assemble_over(&h, inner_at, Tamper::None, usize::MAX)
    } else {
        assemble_real(Tamper::None)
    };
    eprintln!("assembled {point} in {:?}", t0.elapsed());

    let t1 = Instant::now();
    let ok = stark_proofs::witness_satisfies_public(&asm.wired, &asm.witness);
    eprintln!("satisfies in {:?}: {ok}", t1.elapsed());
    if !ok {
        eprintln!("the full-coverage assembly does not satisfy; refusing to emit its shape");
        std::process::exit(1);
    }

    /*
     * The witness is read by the satisfaction walk and by nothing after it. The
     * shape and the layout come from the wired AIR and the layout alone, so the
     * witness goes now rather than sit under the outer periodic root's working set.
     */
    drop(core::mem::take(&mut asm.witness));

    /*
     * The numbers a re-gate compares first, on one line so they cannot be missed.
     * The evaluation domain is 2 * next_pow2(degree * 2^log_trace_len), so the
     * log trace length alone fixes it only while the degree stays at or under 16.
     * The degree is printed beside it rather than assumed, and the domain is
     * derived here from both, so a degree that moved underneath a confirmed log
     * length cannot arrive looking like a sizing success.
     */
    let degree = asm.wired.constraint_degree();
    let log_trace_len = asm.wired.log_trace_len();
    let log_domain = (2 * (degree << log_trace_len).next_power_of_two()).trailing_zeros();
    /*
     * The outer degree is the larger of the region degree and the widest group,
     * plus 2, and on this shape the widest group is what pins it. Its width is
     * printed beside the degree because it is the quantity one step from moving
     * the degree, and the domain with it.
     */
    let max_group_width = asm
        .wired
        .group_params()
        .iter()
        .map(|(cols, _, _)| cols.len())
        .max()
        .unwrap_or(0);
    /*
     * Two query counts, named apart. The inner count is the number of inner
     * proof queries the recursion attests, the per query region blocks in the
     * layout; it is what `n_q` has always been. The outer count is the outer's
     * own FRI query count, a parameter of whoever proves it, not of the
     * assembly, and for the chain it is the settlement point's. A verifier that
     * multiplies per query gas by the wrong one is off by their ratio.
     */
    let outer_n_queries = deployment::N_QUERIES;
    /*
     * The rest of the outer's proving point, emitted rather than assumed. The
     * grind is the proof of work bits the verifier must demand: a proof ground
     * to 16 clears a check of 8, so a verifier that enforced the wrong number
     * would accept identically and only the published bound would be wrong.
     * The blowup sets the outer's evaluation domain, which is (2 * bound) shifted
     * by the extra bits; a rule that derives the domain from the degree and the
     * trace length alone is the rate one half domain, and is short by exactly
     * these bits for a proof at the settlement rate. The log domain is emitted
     * with the extra folded in so nothing downstream derives it without them.
     */
    let grind_bits = deployment::GRIND_BITS;
    let extra_blowup_bits = deployment::EXTRA_BLOWUP_BITS;
    let outer_log_domain = log_domain + extra_blowup_bits;
    /*
     * The composition coefficient count the transcript draws before z: one per
     * transition constraint and one per boundary. A verifier that burns one draw
     * too few or too many squeezes a different z, and every quantity after it,
     * the deep coefficients, the consistency indices, the fold positions,
     * diverges in a way that reads as a bad proof rather than a misconfigured
     * verifier. So the boundary count and the sum are both emitted, and nothing
     * downstream has to reconstruct either.
     */
    let num_transition = asm.wired.num_transition();
    let num_boundary = asm.wired.boundary().len();
    let n_coeffs = num_transition + num_boundary;
    /*
     * The inner's own two counts. The compose region is one fixed-order run, so
     * a verifier that knows where the coefficients start can derive the whole
     * slot map from these rather than carry a transcribed table, and a wrong
     * count moves the derivation instead of failing loudly. They come off the
     * inner AIR the emitter already builds for the periodic root, so nothing
     * downstream has to guess them from the outer's totals.
     */
    let inner_air = stark_proofs::shield_deployed_wired();
    let inner_n_transitions = inner_air.num_transition();
    let inner_n_boundary = inner_air.boundary().len();
    println!(
        "outer     point={point} span={} log_trace_len={log_trace_len} degree={degree} \
         max_group_width={max_group_width} log_domain_rate_half={log_domain} \
         extra_blowup_bits={extra_blowup_bits} log_domain={outer_log_domain} \
         grind_bits={grind_bits} coset_shift={COSET_SHIFT} inner_n_queries={} \
         outer_n_queries={outer_n_queries} trace_width={} num_transition={num_transition} \
         num_boundary={num_boundary} n_coeffs={n_coeffs}",
        asm.lay.span,
        asm.lay.n_q,
        asm.wired.trace_width()
    );

    let js = stark_proofs::shield_deployed_wired();
    let root_extra = if at_transfer { transfer::EXTRA_BLOWUP_BITS } else { inner::extra() };
    let root = stark_proofs::crypto::stark::air::periodic_root_poseidon(&js, root_extra, &h);
    let root_hex: String = root
        .iter()
        .map(|l| format!("{:016x}", l.to_u64()))
        .collect::<Vec<_>>()
        .join("");

    let json = format!(
        "{{\n  \"point\": \"{}\",\n  \"log_trace_len\": {},\n  \"trace_width\": {},\n  \
         \"num_transition\": {},\n  \"num_boundary\": {},\n  \"n_coeffs\": {},\n  \
         \"num_groups\": {},\n  \"constraint_degree\": {},\n  \
         \"inner_log_trace_len\": {},\n  \"inner_trace_width\": {},\n  \"n_queries\": {},\n  \
         \"inner_n_queries\": {},\n  \"outer_n_queries\": {},\n  \"max_group_width\": {},\n  \
         \"grind_bits\": {},\n  \"extra_blowup_bits\": {},\n  \"log_domain\": {},\n  \
         \"coset_shift\": {},\n  \"inner_n_transitions\": {},\n  \
         \"inner_n_boundary\": {},\n  \"outer_fri_queries\": {},\n  \
         \"periodic_root_poseidon\": \"{}\"\n}}\n",
        point,
        asm.wired.log_trace_len(),
        asm.wired.trace_width(),
        num_transition,
        num_boundary,
        n_coeffs,
        asm.n_groups,
        asm.wired.constraint_degree(),
        asm.lay.t_inner.trailing_zeros(),
        asm.lay.width_inner,
        asm.lay.n_q,
        asm.lay.n_q,
        outer_n_queries,
        max_group_width,
        grind_bits,
        extra_blowup_bits,
        outer_log_domain,
        COSET_SHIFT,
        inner_n_transitions,
        inner_n_boundary,
        /*
         * The same value as `outer_n_queries` under the name that says which
         * FRI it counts. The old key stays for one release so a reader pinned
         * to it does not break; after that the ambiguous name goes, because a
         * count of 32 sitting beside a 64 query transfer point is exactly the
         * pair a verifier multiplies the wrong way round.
         */
        outer_n_queries,
        root_hex,
    );
    std::fs::write(&out, &json).expect("write structure");
    println!("{json}");
    println!("wrote {out}");

    /*
     * The region layout, for a verifier whose evaluators are config-driven over
     * it: every Layout scalar, the region bases, and the permutation's committed
     * geometry. Same discipline as the shape file: derived, gated by the
     * satisfaction walk above, never typed.
     */
    let lay = &asm.lay;
    /*
     * The outer's own periodic root is the one allocation in this section that
     * scales with the outer: it extends every periodic column to the evaluation
     * domain before hashing. It is computed first and announced on both sides,
     * so if the machine cannot hold it the run says where it died, rather than
     * ending after the shape with no layout and no message.
     */
    eprintln!("computing the outer periodic root");
    let outer_root_hex: String = {
        let r = stark_proofs::crypto::stark::air::periodic_root(&asm.wired, 0);
        r.iter().map(|b| format!("{b:02x}")).collect()
    };
    /*
     * The outer's own periodic columns: what a query's periodic row carries,
     * and what the chain opens against the keccak root above. This is not
     * n_pz. That one counts the inner's periodic columns, which the recursion
     * checks in circuit against the Poseidon root in the shape. Two
     * commitments over two column sets, under two hashes, and reading one for
     * the other fails every opening, so both are emitted side by side.
     */
    let outer_n_periodic = asm.wired.periodic_columns().len();
    eprintln!("outer periodic root done, {outer_n_periodic} outer periodic columns");
    /*
     * The kind map: which constraint body each kind runs, where its periodic
     * values begin, and how wide its body is. This is the one thing about the
     * outer that a verifier cannot derive from any other emitted number, because
     * it is a statement about how the assembler pushed its regions rather than a
     * property of the trace.
     */
    let kmap = asm.wired.kind_map();
    /*
     * The widest arity among the kinds that are not compose. The widest arity
     * overall is already `group_constraint_base` under another name, because
     * the fused AIR takes the maximum arity as its region lane count and then
     * appends one lane per group, so emitting it again would be the same
     * subtraction twice with two names. What is not derivable is where compose
     * stops being the only contributor: every kind contributes at every index
     * below its own arity, so a verifier that dispatches per lane needs to know
     * how far up the vector a second body can still reach. Compose is named
     * rather than inferred from the width, because a body being widest is a
     * fact about this circuit and not a definition.
     */
    let max_noncompose_arity = kmap
        .iter()
        .zip(&asm.kind_bodies)
        .filter(|(_, (body, _))| *body != "compose")
        .map(|(&(_, _, arity, _), _)| arity)
        .max()
        .unwrap_or(0);
    assert_eq!(
        kmap.len(),
        asm.kind_bodies.len(),
        "every kind must name the body it runs"
    );
    let kinds_json: Vec<String> = kmap
        .iter()
        .zip(&asm.kind_bodies)
        .enumerate()
        .map(|(k, (&(base, slots, arity, instances), (body, role)))| {
            format!(
                "{{\"kind\": {k}, \"body\": \"{body}\", \"role\": \"{role}\", \
                 \"periodic_base\": {base}, \"slots\": {slots}, \"arity\": {arity}, \
                 \"instances\": {instances}}}"
            )
        })
        .collect();
    /*
     * A kind that owns no periodic columns leaves the next kind starting at the
     * same base, so the base list is not injective and cannot identify a kind at
     * all. Counted here rather than left as a remark, so a reader who wants to
     * know whether the property still holds can read a number instead of
     * rediscovering it.
     */
    let base_collisions = kmap
        .iter()
        .enumerate()
        .filter(|(i, &(base, _, _, _))| {
            kmap.iter().take(*i).any(|&(other, _, _, _)| other == base)
        })
        .count();
    eprintln!("reading the permutation columns");
    let (sel_idx, row_idx, sig_base) = asm.wired.permutation_columns();
    eprintln!("permutation columns read; formatting {} groups", sig_base.len());
    let groups_json: Vec<String> = asm
        .wired
        .group_params()
        .iter()
        .zip(&sig_base)
        .map(|((cols, beta, gamma), sb)| {
            format!(
                "{{\"wired_cols\": {:?}, \"beta\": {}, \"gamma\": {}, \"sigma_base_col\": {}}}",
                cols,
                beta.to_u64(),
                gamma.to_u64(),
                sb
            )
        })
        .collect();
    let layout = format!(
        "{{\n  \"span\": {},\n  \"l\": {},\n  \"n_q\": {},\n  \"region_offsets\": {:?},\n  \
         \"z_op\": {},\n  \"claim_op\": {},\n  \"deep_coeff_op\": {},\n  \"pub_len\": {},\n  \
         \"ntr\": {},\n  \"ncoeff2\": {},\n  \"n_terms\": {},\n  \"width_inner\": {},\n  \
         \"window_inner\": {},\n  \"depth\": {},\n  \"n_open\": {},\n  \"n_folds\": {},\n  \
         \"log_n_inner\": {},\n  \"pbits\": {},\n  \"fbits\": {},\n  \"t_inner\": {},\n  \
         \"n_pz\": {},\n  \"pa_depth\": {},\n  \"n_chunks\": {},\n  \"frame_len\": {},\n  \
         \"n_coeff\": {},\n  \"c_periodic_col\": {},\n  \"c_z_col\": {},\n  \"c_coeff_col\": {},\n  \
         \"c_comp_z_col\": {},\n  \"sel_col\": {},\n  \"row_col\": {},\n  \
         \"strip_off\": {},\n  \"strip_k\": {},\n  \"strip_echo_width\": {},\n  \
         \"strip_n_out\": {},\n  \"strip_rows\": {},\n  \"outer_n_periodic\": {},\n  \
         \"outer_periodic_root_keccak\": \"{}\",\n  \
         \"max_noncompose_arity\": {},\n  \"periodic_base_collisions\": {},\n  \
         \"group_column_base\": {},\n  \
         \"group_constraint_base\": {},\n  \
         \"kinds\": [\n    {}\n  ],\n  \
         \"groups\": [\n    {}\n  ]\n}}\n",
        lay.span,
        lay.l,
        lay.n_q,
        asm.region_offsets,
        lay.z_op,
        lay.claim_op,
        lay.deep_coeff_op,
        lay.pub_len,
        lay.ntr,
        lay.ncoeff2,
        lay.n_terms,
        lay.width_inner,
        lay.window_inner,
        lay.depth,
        lay.n_open,
        lay.n_folds,
        lay.log_n,
        lay.pbits,
        lay.fbits,
        lay.t_inner,
        lay.n_pz,
        lay.pa_depth,
        lay.n_chunks,
        lay.frame_len,
        lay.n_coeff,
        lay.c_periodic_col,
        lay.c_z_col,
        lay.c_coeff_col,
        lay.c_comp_z_col,
        sel_idx,
        row_idx,
        lay.strip_off,
        lay.strip_k,
        lay.strip_echo_width,
        lay.strip_n_out,
        lay.strip_rows,
        outer_n_periodic,
        outer_root_hex,
        max_noncompose_arity,
        base_collisions,
        /*
         * Two bases with the same arithmetic shape over different vectors, so
         * both are named rather than left to be inferred. The column base is
         * where the grand product running product columns begin in the trace:
         * each group owns one and they sit at the end, so it is the trace width
         * less the group count. The constraint base is where those groups' lanes
         * begin in the constraint vector: the transitions the regions write,
         * less the same count. Reading one where the other was meant lands in
         * the wrong vector entirely, and both read as "something minus the
         * group count", which is exactly why they are emitted apart.
         */
        asm.wired.trace_width() - asm.n_groups,
        num_transition - asm.n_groups,
        kinds_json.join(",\n    "),
        groups_json.join(",\n    "),
    );
    let lay_out = out.replace(".json", "-layout.json");
    eprintln!("layout formatted, {} bytes; writing {lay_out}", layout.len());
    std::fs::write(&lay_out, &layout).expect("write layout");
    println!("wrote {lay_out}");
}
