// NONOS Operating System (AGPL-3.0-or-later)
//! The deployed inner's own layout: its boundary constraints and its region
//! structure.
//!
//! Both ride the outer's structure emit as well, but that run assembles the
//! whole outer and commits its periodic tree at the deployment rate, which is
//! most of two hours. Everything here comes from the inner AIR alone and costs
//! a second, and a verifier that cannot recompute the inner's transitions is
//! blocked on exactly this, so it is worth handing over without waiting.
//!
//! What a consumer needs to recompute an inner transition is the same set the
//! outer's kind map carries, for the same reason: the selector column says
//! which kind a row belongs to and nothing says which rule that kind enforces,
//! or how wide its window is, or where its periodic values begin. None of it is
//! recoverable from the trace.
//!
//! Boundary order is the inner engine's own, which is the order the
//! composition coefficients were drawn against. A consumer pairs the two by
//! index and neither side has to agree about sorting.

use stark_proofs::crypto::stark::air::Air;

fn main() {
    let gen = stark_proofs::shield_deployed_wired();
    let air = gen.wired();
    let boundary = Air::boundary(air);

    let triples: Vec<String> = boundary
        .iter()
        .map(|(col, row, val)| format!("[{col}, {row}, {}]", val.to_u64()))
        .collect();

    /*
     * Per kind: which rule it enforces, where its periodic values begin, how
     * many it owns, how many constraint indices it writes, how many regions run
     * it, and how wide one of those regions is.
     *
     * The body is the field that cannot be inferred. Two of the seven are
     * memberships and two are Poseidon chains of equal width, so neither the
     * index nor the shape identifies a kind, and a reader dispatching on the
     * index against an older assumption computes a different transition rather
     * than failing. The width is what lets a caller slice the window at all:
     * the widths are not uniform and the window is two rows.
     */
    let kinds: Vec<String> = air
        .kind_map()
        .iter()
        .enumerate()
        .map(|(k, &(base, slots, arity, instances, width))| {
            let body = stark_proofs::shield::batch::KIND_BODIES
                .get(k)
                .copied()
                .unwrap_or("unknown");
            format!(
                "{{\"kind\": {k}, \"body\": \"{body}\", \"periodic_base\": {base}, \
                 \"slots\": {slots}, \"arity\": {arity}, \"instances\": {instances}, \
                 \"region_width\": {width}}}"
            )
        })
        .collect();

    /*
     * The permutation's committed geometry: the product selector and the row
     * counter are shared by every group, so they are emitted once, and only
     * the sigma base is per group.
     */
    let (sel_col, row_col, sig_base) = gen.permutation_columns();
    let groups: Vec<String> = gen
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

    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "inner-layout.json".into());
    let json = format!(
        "{{\n  \"point\": \"settlement\",\n  \"inner_trace_width\": {},\n  \
         \"inner_log_trace_len\": {},\n  \"inner_n_transitions\": {},\n  \
         \"inner_n_boundary\": {},\n  \"inner_constraint_degree\": {},\n  \
         \"inner_window_size\": {},\n  \"inner_n_kinds\": {},\n  \
         \"inner_sel_col\": {},\n  \"inner_row_col\": {},\n  \"inner_n_groups\": {},\n  \
         \"inner_boundary\": [\n    {}\n  ],\n  \
         \"inner_kinds\": [\n    {}\n  ],\n  \
         \"inner_groups\": [\n    {}\n  ]\n}}\n",
        Air::trace_width(air),
        Air::log_trace_len(air),
        Air::num_transition(air),
        boundary.len(),
        Air::constraint_degree(air),
        Air::window_size(air),
        kinds.len(),
        sel_col,
        row_col,
        groups.len(),
        triples.join(",\n    "),
        kinds.join(",\n    "),
        groups.join(",\n    "),
    );
    std::fs::write(&out, &json).expect("write inner layout");
    println!("{json}");
    println!("wrote {out}");
}
