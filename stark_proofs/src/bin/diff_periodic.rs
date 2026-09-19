// NONOS Operating System (AGPL-3.0-or-later)
//! Which periodic columns two points disagree on, and which kind owns them.
//!
//! The outer over a live spend has the settlement outer's shape exactly:
//! same width, same degree, same transition count, same group count. It does
//! not have the settlement outer's periodic root, which is how a proof made
//! for one came to fail against the other. Shape is evidently not the whole
//! of what the periodic set depends on, and the question is what else.
//!
//! This assembles both and walks their periodic columns together, reporting
//! the first index that differs and the kind whose slots contain it. The
//! answer decides something much larger than this proof: if the periodic set
//! depends on the inner proof's content rather than only on its shape, then
//! a deployed verifier is specific to one statement, and every spend needs
//! its own verifier. That would be a finding about the architecture, not
//! about tonight.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::recursion_assembly::point::Point;
use std::time::Instant;

fn main() {
    let t0 = Instant::now();
    let a = Point::Settlement.assemble().expect("settlement assembles");
    eprintln!("assembled settlement in {:?}", t0.elapsed());
    let t1 = Instant::now();
    let b = Point::Spend {
        recipient: 0x7408_ae4c,
        clearing_price: 1_000_000,
    }
    .assemble()
    .expect("the spend assembles");
    eprintln!("assembled spend in {:?}", t1.elapsed());

    /*
     * The test that actually decides deployability, and the one the
     * contracts lane insisted on: a second spend differing in every way a
     * production spend differs. Two spends against the same root would share
     * a periodic set even on a circuit that baked the tree in, so only this
     * comparison can tell a deployable design from an undeployable one.
     */
    let t2 = Instant::now();
    let h = stark_proofs::recursion_assembly::inner::hasher();
    let v = stark_proofs::recursion_assembly::inner::shield_join_split_of(
        &h,
        stark_proofs::shield::live::unshield_variant(),
        None,
    );
    let c = stark_proofs::recursion_assembly::build::assemble_over(
        &h,
        v,
        stark_proofs::recursion_assembly::Tamper::None,
        usize::MAX,
    );
    eprintln!("assembled the variant spend in {:?}", t2.elapsed());
    let (pb2, pc) = (b.wired.periodic_columns(), c.wired.periodic_columns());
    if pb2.len() != pc.len() {
        println!(
            "SPEND vs VARIANT: column counts differ, {} against {}",
            pb2.len(),
            pc.len()
        );
    } else {
        let d = pb2.iter().zip(pc.iter()).filter(|(x, y)| x != y).count();
        println!(
            "SPEND vs VARIANT: {d} of {} periodic columns differ",
            pb2.len()
        );
        if d == 0 {
            println!(
                "  so two spends differing in root, leaf indices, siblings, values and \
                 recipient share one circuit: ONE VERIFIER SERVES EVERY SPEND"
            );
        } else {
            println!(
                "  so a spend's circuit depends on which notes it spends: EVERY SPEND \
                 WOULD NEED ITS OWN VERIFIER"
            );
        }
    }

    println!(
        "settlement  width {} log_t {} transitions {} groups {}",
        Air::trace_width(&a.wired),
        Air::log_trace_len(&a.wired),
        Air::num_transition(&a.wired),
        a.n_groups
    );
    println!(
        "spend       width {} log_t {} transitions {} groups {}",
        Air::trace_width(&b.wired),
        Air::log_trace_len(&b.wired),
        Air::num_transition(&b.wired),
        b.n_groups
    );

    /*
     * The outer's wiring is a function of its Layout and nothing else, so if
     * sigma columns differ the Layout must. Printed field by field, because
     * the field that moved names the cause.
     */
    let (la, lb) = (&a.lay, &b.lay);
    let fields: [(&str, usize, usize); 14] = [
        ("span", la.span, lb.span),
        ("l", la.l, lb.l),
        ("n_q", la.n_q, lb.n_q),
        ("depth", la.depth, lb.depth),
        ("n_open", la.n_open, lb.n_open),
        ("n_folds", la.n_folds, lb.n_folds),
        ("width_inner", la.width_inner, lb.width_inner),
        ("window_inner", la.window_inner, lb.window_inner),
        ("n_pz", la.n_pz, lb.n_pz),
        ("pa_depth", la.pa_depth, lb.pa_depth),
        ("frame_len", la.frame_len, lb.frame_len),
        ("n_coeff", la.n_coeff, lb.n_coeff),
        ("pub_len", la.pub_len, lb.pub_len),
        ("n_terms", la.n_terms, lb.n_terms),
    ];
    let mut moved = 0;
    for (name, x, y) in fields {
        if x != y {
            println!("LAYOUT DIFFERS  {name}: settlement {x}, spend {y}");
            moved += 1;
        }
    }
    if moved == 0 {
        println!("every compared Layout field is identical");
    }
    if a.region_offsets != b.region_offsets {
        let first = a
            .region_offsets
            .iter()
            .zip(b.region_offsets.iter())
            .position(|(x, y)| x != y);
        println!("REGION OFFSETS DIFFER, first at index {first:?}");
    } else {
        println!(
            "region offsets identical ({} regions)",
            a.region_offsets.len()
        );
    }

    let pa = a.wired.periodic_columns();
    let pb = b.wired.periodic_columns();
    println!(
        "periodic columns: settlement {} spend {}",
        pa.len(),
        pb.len()
    );
    if pa.len() != pb.len() {
        println!("the counts differ, so the sets are not comparable column by column");
        return;
    }

    /*
     * Which kind a column belongs to, from the kind map's periodic bases. A
     * differing column is only useful if it can be named, and the base list
     * is what names it.
     */
    let kmap = a.wired.kind_map();
    let owner = |col: usize| -> String {
        let mut best = ("none", usize::MAX);
        for ((k, &(base, slots, _, _, _)), (body, role)) in
            kmap.iter().enumerate().zip(&a.kind_bodies)
        {
            if col >= base && col < base + slots && base <= col {
                let _ = k;
                best = (body, base);
                let _ = role;
            }
        }
        if best.1 == usize::MAX {
            "outside every kind's slots (sigma, selector or shared)".into()
        } else {
            format!("kind body {}, slots from {}", best.0, best.1)
        }
    };

    let mut differing: Vec<usize> = Vec::new();
    for (i, (ca, cb)) in pa.iter().zip(pb.iter()).enumerate() {
        if ca != cb {
            differing.push(i);
        }
    }

    println!("columns that differ: {}", differing.len());
    for &i in differing.iter().take(12) {
        let first = pa[i]
            .iter()
            .zip(pb[i].iter())
            .position(|(x, y)| x != y)
            .unwrap_or(usize::MAX);
        println!(
            "  column {i:5}  first differing row {first}  ({})",
            owner(i)
        );
    }
    if differing.len() > 12 {
        println!("  ... and {} more", differing.len() - 12);
    }
    if differing.is_empty() {
        println!(
            "the periodic sets are identical, so the failing verify is not a \
             periodic root disagreement and the cause is elsewhere"
        );
    }
}
