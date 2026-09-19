// NONOS Operating System (AGPL-3.0-or-later)
//! The outer's wiring, as a partition and as one digest.
//!
//! The permutation is argued today as 251 narrow grand products, each with
//! its own sigma columns, because a product over `k` columns is degree
//! `k + 1` and the circuit has a degree budget. That cut is an
//! implementation choice: on the settlement outer it costs 2,013 sigma
//! columns to describe wiring over 362 distinct columns, a duplication of
//! five and a half, and those columns are three quarters of everything a
//! chunk re-sends and re-hashes on chain.
//!
//! Replacing it with one argument and a chained accumulator has to prove one
//! thing before it proves anything else: the wiring did not move. That is
//! what this emits. The classes are the cells the binds hold equal, in
//! canonical order, and the digest is over all of them. A redesign that
//! reproduces the digest argues the same statement by a cheaper route; one
//! that does not has changed the circuit, and the per class counts and the
//! first differing cell say where.
//!
//! No proving and no hashing of a trace: this is the assembly's binds and a
//! union-find, minutes rather than hours.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::crypto::stark::hash::keccak256;
use stark_proofs::recursion_assembly::groups::wiring_classes;
use stark_proofs::recursion_assembly::point::Point;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let point = Point::from_args(&args);
    let out = args
        .iter()
        .find(|a| !Point::is_flag(a))
        .cloned()
        .unwrap_or_else(|| format!("{}-wiring.json", point.name()));

    let t0 = Instant::now();
    let (asm, binds) = match point.assemble_with_binds() {
        Ok(pair) => pair,
        Err(why) => {
            eprintln!("{why}");
            std::process::exit(2);
        }
    };
    eprintln!("assembled {} in {:?}", point.name(), t0.elapsed());

    let span = asm.lay.span;
    let width = Air::trace_width(&asm.wired);
    let t1 = Instant::now();
    let classes = wiring_classes(&binds, span, width);
    eprintln!("closed {} classes in {:?}", classes.len(), t1.elapsed());

    /*
     * One digest over the whole partition, absorbed in canonical order with
     * the class boundaries inside the hash, so two partitions that differ
     * only in where a class ends cannot collide. The span and width go in
     * first: the same cell index means a different cell under a different
     * width.
     */
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"NONOS-OUTER-WIRING-V1");
    buf.extend_from_slice(&(span as u64).to_le_bytes());
    buf.extend_from_slice(&(width as u64).to_le_bytes());
    buf.extend_from_slice(&(classes.len() as u64).to_le_bytes());
    for class in &classes {
        buf.extend_from_slice(&(class.len() as u64).to_le_bytes());
        for &cell in class {
            buf.extend_from_slice(&(cell as u64).to_le_bytes());
        }
    }
    let digest: String = keccak256(&buf).iter().map(|b| format!("{b:02x}")).collect();

    let cells: usize = classes.iter().map(Vec::len).sum();
    let mut columns: Vec<usize> = classes
        .iter()
        .flat_map(|c| c.iter().map(|&cell| cell % width))
        .collect();
    columns.sort_unstable();
    columns.dedup();
    let largest = classes.iter().map(Vec::len).max().unwrap_or(0);

    /*
     * The class sizes as a histogram rather than the classes themselves: the
     * partition is millions of cells and a reader wants the shape, while the
     * digest above is what a gate compares. The full partition goes to the
     * sidecar file below for a diff that has to name a cell.
     */
    let mut sizes: Vec<(usize, usize)> = Vec::new();
    for class in &classes {
        match sizes.iter_mut().find(|(s, _)| *s == class.len()) {
            Some((_, n)) => *n += 1,
            None => sizes.push((class.len(), 1)),
        }
    }
    sizes.sort_unstable();

    let json = format!(
        "{{\n  \"point\": \"{}\",\n  \"span\": {span},\n  \"trace_width\": {width},\n  \
         \"n_classes\": {},\n  \"n_cells\": {cells},\n  \"n_wired_columns\": {},\n  \
         \"largest_class\": {largest},\n  \
         \"class_sizes\": [{}],\n  \
         \"wiring_digest_keccak\": \"{digest}\"\n}}\n",
        point.name(),
        classes.len(),
        columns.len(),
        sizes
            .iter()
            .map(|(s, n)| format!("[{s}, {n}]"))
            .collect::<Vec<_>>()
            .join(", "),
    );
    std::fs::write(&out, &json).expect("write wiring");
    println!("{json}");
    println!("wrote {out}");

    /*
     * The partition itself, one class per line, cells separated by spaces.
     * Plain text so a differing run is `diff` and the first differing line is
     * the first class that moved, without a parser in between.
     */
    let full = out.replace(".json", "-classes.txt");
    let mut body = String::new();
    for class in &classes {
        for (i, cell) in class.iter().enumerate() {
            if i > 0 {
                body.push(' ');
            }
            body.push_str(&cell.to_string());
        }
        body.push('\n');
    }
    std::fs::write(&full, &body).expect("write classes");
    println!("wrote {full}, {} bytes", body.len());
}
