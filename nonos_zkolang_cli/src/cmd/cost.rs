/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `check --cost` (section 15.2): the rows of each function, inclusive of the functions
 * inlined into it and exclusive of them, the most registers live at a row of its own, and
 * the source lines whose code takes the most rows.
 */

use std::cmp::Reverse;
use std::collections::BTreeMap;

use nonos_zkolang::compiler::driver::{cost, Built};
use nonos_zkolang::compiler::source::SourceMap;

/** How many lines the report names. */
const TOP: usize = 10;

/** Print the cost report of `b`, whose files are in `map`. */
pub(super) fn print_cost(map: &SourceMap, b: &Built) {
    let c = cost(b);
    println!(
        "{} rows, {} outside any function; at most {} registers live",
        c.rows, c.overhead, c.peak
    );
    println!(
        "{:>9} {:>9} {:>5}  function",
        "inclusive", "exclusive", "peak"
    );
    let mut fns: Vec<_> = c.fns.iter().collect();
    fns.sort_by_key(|(f, k)| (Reverse(k.inclusive), **f));
    for (f, k) in fns {
        let name = b
            .program
            .fns
            .get(f.0 as usize)
            .map_or("?", |x| x.name.as_str());
        println!(
            "{:>9} {:>9} {:>5}  {name}",
            k.inclusive, k.exclusive, k.peak
        );
    }
    let mut lines: BTreeMap<(u32, usize), usize> = BTreeMap::new();
    for (span, n) in &c.spans {
        let row = map.file(span.file).map_or(0, |f| f.line_col(span.lo).0);
        let e = lines.entry((span.file.0, row)).or_default();
        *e = e.saturating_add(*n);
    }
    let mut top: Vec<_> = lines.into_iter().collect();
    top.sort_by_key(|&(at, n)| (Reverse(n), at));
    println!("the lines that take the most rows");
    for ((file, row), n) in top.into_iter().take(TOP) {
        let f = map.files().get(file as usize);
        let (name, text) = f.map_or(("?", ""), |f| (f.name.as_str(), f.line_text(row)));
        println!("{n:>9}  {name}:{row}  {}", text.trim());
    }
}
