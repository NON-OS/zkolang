// NONOS Operating System (AGPL-3.0-or-later)
//! Which binding family, and which layout vector, two spends disagree on.
//!
//! Spends at different leaf positions have different circuits, measured: the
//! periodic columns differ and every scalar layout field and all 228 region
//! offsets are identical. The wiring is therefore varying on something the
//! scalar fields do not carry, and the candidates are the layout's vector
//! fields, which describe where each inner's cells sit.
//!
//! Reading did not settle it. The shield's note membership pins the bottom
//! direction, so its opened cell is at a fixed column rather than one the
//! direction selects; the nullifier region's openings are all direction
//! false; and `IndexScalar` has periodic columns of powers of two and a
//! single zero boundary, none of which depend on the index. So the
//! specialisation is somewhere those three are not, and this asks the
//! assembly rather than the source.
//!
//! `Bind` carries a label for exactly this, so a differing family can be
//! named rather than inferred from a column number.

use stark_proofs::recursion_assembly::build::binds_for;
use stark_proofs::recursion_assembly::inner;
use stark_proofs::recursion_assembly::{build::assemble_over, Tamper};
use std::collections::BTreeMap;

fn main() {
    let h = inner::hasher();
    let a = assemble_over(
        &h,
        inner::shield_join_split_of(
            &h,
            stark_proofs::shield::live::unshield_at(0, 1, 4, 1_000_000_000_000),
            None,
        ),
        Tamper::None,
        usize::MAX,
    );
    let b = assemble_over(
        &h,
        inner::shield_join_split_of(
            &h,
            stark_proofs::shield::live::unshield_at(2, 3, 4, 1_000_000_000_000),
            None,
        ),
        Tamper::None,
        usize::MAX,
    );
    eprintln!("both assembled");

    let (la, lb) = (&a.lay, &b.lay);

    macro_rules! vecfield {
        ($name:literal, $f:ident) => {
            if la.$f != lb.$f {
                let first = la.$f.iter().zip(lb.$f.iter()).position(|(x, y)| x != y);
                println!(
                    "LAYOUT VEC DIFFERS  {:16} lengths {} vs {}, first differing element {:?}",
                    $name,
                    la.$f.len(),
                    lb.$f.len(),
                    first
                );
            } else {
                println!(
                    "same                {:16} ({} elements)",
                    $name,
                    la.$f.len()
                );
            }
        };
    }

    vecfield!("i_off", i_off);
    vecfield!("d_off", d_off);
    vecfield!("f_off", f_off);
    vecfield!("m_off", m_off);
    vecfield!("fp_off", fp_off);
    vecfield!("ta_off", ta_off);
    vecfield!("pa_off", pa_off);
    vecfield!("ocells", ocells);
    vecfield!("pchunk_cells", pchunk_cells);
    vecfield!("tchunk_cells", tchunk_cells);
    vecfield!("strip_cycles", strip_cycles);

    /*
     * The binds themselves, grouped by the label each family carries. Two
     * spends whose layouts agree must produce identical binds; if they do
     * not, the family that moved is the one specialising on the position.
     */
    let (ba, bb) = (binds_for(la), binds_for(lb));
    println!("\nbinds: {} vs {}", ba.len(), bb.len());
    let mut by_label: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for g in &ba {
        by_label.entry(g.label).or_default().0 += 1;
    }
    for g in &bb {
        by_label.entry(g.label).or_default().1 += 1;
    }
    for (label, (x, y)) in &by_label {
        let name = if label.is_empty() {
            "(unlabelled)"
        } else {
            label
        };
        println!(
            "  {name:12} {x} vs {y}{}",
            if x == y { "" } else { "   COUNT DIFFERS" }
        );
    }

    let mut moved = 0usize;
    for (i, (x, y)) in ba.iter().zip(bb.iter()).enumerate() {
        if x.wired_cols != y.wired_cols || x.swaps != y.swaps {
            if moved < 8 {
                let name = if x.label.is_empty() {
                    "(unlabelled)"
                } else {
                    x.label
                };
                println!(
                    "  bind {i:4} differs  label {name}  cols {} vs {}  swaps {} vs {}",
                    x.wired_cols.len(),
                    y.wired_cols.len(),
                    x.swaps.len(),
                    y.swaps.len()
                );
            }
            moved += 1;
        }
    }
    println!("binds that differ: {moved}");
    if moved == 0 {
        println!(
            "the binds are identical, so the wiring is not what moved and the \
             difference entered after collapse"
        );
    }
}
