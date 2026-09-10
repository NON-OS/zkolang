// NONOS Operating System (AGPL-3.0-or-later)
//! From plan to layout: every witnessed product becomes a lane on a row, and
//! each of its two operands becomes a linear form: coefficients over window
//! slots and echo cells, plus a constant. Constant subtrees evaluate to field
//! elements and fold into the coefficients, which is where the 1977 scalar
//! edges go. The layout is a pure function of the tape, and it is gated by
//! evaluation before any AIR is built from it.

use super::pack::pack;
use super::replay::const_mask;
use super::schedule::producers_of;
use super::tape::Node;
use crate::crypto::stark::field::Fp;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Source {
    /// A product slot in the window: this row (1) or the row above (0).
    Slot { rel: u8, lane: u16 },
    /// An echo cell on this row, cycled in from a distant product or input.
    Echo { idx: u16 },
}

#[derive(Clone, Debug)]
pub struct LinForm {
    pub terms: Vec<(Fp, Source)>,
    pub constant: Fp,
}

#[derive(Clone, Debug)]
pub struct Lane {
    pub node: u32,
    pub a: LinForm,
    pub b: LinForm,
}

#[derive(Clone, Debug, Default)]
pub struct Row {
    pub lanes: Vec<Lane>,
    /// What each echo cell on this row carries: a witnessed product's tape
    /// id or an input's tape id, filled by copy cycle.
    pub echoes: Vec<u32>,
}

pub struct StripLayout {
    pub rows: Vec<Row>,
    /// Output j as a linear form over (row, source) pairs plus a constant:
    /// the accumulator schedule, one contribution list per row.
    pub out_terms: Vec<Vec<(usize, Fp, Source)>>,
    pub out_consts: Vec<Fp>,
    pub k: usize,
}

/// The linear form of a node over witnessed products and inputs, with
/// constants folded: id-keyed, before sources are assigned.
fn linear(
    tape: &[Node],
    cmask: &[bool],
    cval: &mut BTreeMap<u32, Fp>,
    cache: &mut BTreeMap<u32, (BTreeMap<u32, Fp>, Fp)>,
    n: u32,
) -> (BTreeMap<u32, Fp>, Fp) {
    if let Some(hit) = cache.get(&n) {
        return hit.clone();
    }
    let r = match &tape[n as usize] {
        Node::Input(_) => (BTreeMap::from([(n, Fp::ONE)]), Fp::ZERO),
        Node::Const(c) => (BTreeMap::new(), *c),
        Node::Add(a, b) | Node::Sub(a, b) => {
            let (ma, ca) = linear(tape, cmask, cval, cache, *a);
            let (mb, cb) = linear(tape, cmask, cval, cache, *b);
            let sub = matches!(&tape[n as usize], Node::Sub(_, _));
            let mut m = ma;
            for (k, v) in mb {
                let e = m.entry(k).or_insert(Fp::ZERO);
                *e = if sub { *e - v } else { *e + v };
            }
            (m, if sub { ca - cb } else { ca + cb })
        }
        Node::Mul(a, b) => {
            let (ca, cb) = (cmask[*a as usize], cmask[*b as usize]);
            if ca && cb {
                (BTreeMap::new(), constant_of(tape, cval, n))
            } else if ca {
                let s = constant_of(tape, cval, *a);
                scaled(tape, cmask, cval, cache, *b, s)
            } else if cb {
                let s = constant_of(tape, cval, *b);
                scaled(tape, cmask, cval, cache, *a, s)
            } else {
                (BTreeMap::from([(n, Fp::ONE)]), Fp::ZERO)
            }
        }
    };
    cache.insert(n, r.clone());
    r
}

fn scaled(
    tape: &[Node],
    cmask: &[bool],
    cval: &mut BTreeMap<u32, Fp>,
    cache: &mut BTreeMap<u32, (BTreeMap<u32, Fp>, Fp)>,
    n: u32,
    s: Fp,
) -> (BTreeMap<u32, Fp>, Fp) {
    let (m, c) = linear(tape, cmask, cval, cache, n);
    (m.into_iter().map(|(k, v)| (k, v * s)).collect(), c * s)
}

/// A constant subtree's value, iteratively memoized.
fn constant_of(tape: &[Node], cval: &mut BTreeMap<u32, Fp>, n: u32) -> Fp {
    if let Some(v) = cval.get(&n) {
        return *v;
    }
    let mut stack = alloc::vec![n];
    while let Some(&x) = stack.last() {
        if cval.contains_key(&x) {
            stack.pop();
            continue;
        }
        let v = match &tape[x as usize] {
            Node::Const(c) => Some(*c),
            Node::Input(_) => panic!("constant_of on a variable"),
            Node::Add(a, b) | Node::Sub(a, b) | Node::Mul(a, b) => {
                match (cval.get(a), cval.get(b)) {
                    (Some(va), Some(vb)) => Some(match &tape[x as usize] {
                        Node::Add(_, _) => *va + *vb,
                        Node::Sub(_, _) => *va - *vb,
                        _ => *va * *vb,
                    }),
                    _ => {
                        stack.push(*a);
                        stack.push(*b);
                        None
                    }
                }
            }
        };
        if let Some(v) = v {
            cval.insert(x, v);
            stack.pop();
        }
    }
    cval[&n]
}

/// Assign sources on a row: window slots for products born here or one row
/// up, echo cells for everything else, inputs always by echo.
fn to_sources(
    ids: &BTreeMap<u32, Fp>,
    row: usize,
    place: &BTreeMap<u32, (usize, u16)>,
    echoes: &mut Vec<u32>,
) -> Vec<(Fp, Source)> {
    let mut out = Vec::with_capacity(ids.len());
    for (id, coeff) in ids {
        let src = match place.get(id) {
            Some((r, lane)) if *r == row => Source::Slot { rel: 1, lane: *lane },
            Some((r, lane)) if *r + 1 == row => Source::Slot { rel: 0, lane: *lane },
            _ => {
                let idx = echoes.iter().position(|e| e == id).unwrap_or_else(|| {
                    echoes.push(*id);
                    echoes.len() - 1
                });
                Source::Echo { idx: idx as u16 }
            }
        };
        out.push((*coeff, src));
    }
    out
}

/// Build the layout: pack at `k`, extract every lane's linear forms, assign
/// window and echo sources, and lay the outputs' accumulator schedule.
pub fn strip_layout(tape: &[Node], outputs: &[u32], k: usize) -> StripLayout {
    let cmask = const_mask(tape);
    let pk = pack(tape, outputs, k);
    let mut cval: BTreeMap<u32, Fp> = BTreeMap::new();
    let mut cache: BTreeMap<u32, (BTreeMap<u32, Fp>, Fp)> = BTreeMap::new();
    let _ = producers_of(tape);

    // Lane placement: order within a row follows the packing order.
    let mut place: BTreeMap<u32, (usize, u16)> = BTreeMap::new();
    let mut per_row: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
    for (n, r) in &pk.rows_of {
        per_row.entry(*r).or_default().push(*n);
    }
    for (r, ns) in &per_row {
        for (lane, n) in ns.iter().enumerate() {
            place.insert(*n, (*r, lane as u16));
        }
    }

    let mut rows: Vec<Row> = alloc::vec![Row::default(); pk.rows];
    for (n, (r, _)) in &place {
        if let Node::Mul(a, b) = &tape[*n as usize] {
            let (ma, ca) = linear(tape, &cmask, &mut cval, &mut cache, *a);
            let (mb, cb) = linear(tape, &cmask, &mut cval, &mut cache, *b);
            let row = &mut rows[*r];
            let a_form = LinForm {
                terms: to_sources(&ma, *r, &place, &mut row.echoes),
                constant: ca,
            };
            let b_form = LinForm {
                terms: to_sources(&mb, *r, &place, &mut row.echoes),
                constant: cb,
            };
            row.lanes.push(Lane { node: *n, a: a_form, b: b_form });
        }
    }

    // Outputs: each is a linear form whose product terms land on the row the
    // product is born (the accumulator absorbs them there); input and
    // constant terms land on row 0.
    let mut out_terms: Vec<Vec<(usize, Fp, Source)>> = Vec::with_capacity(outputs.len());
    let mut out_consts: Vec<Fp> = Vec::with_capacity(outputs.len());
    for out in outputs {
        let (m, c) = linear(tape, &cmask, &mut cval, &mut cache, *out);
        let mut terms = Vec::with_capacity(m.len());
        for (id, coeff) in m {
            match place.get(&id) {
                Some((r, lane)) => {
                    terms.push((*r, coeff, Source::Slot { rel: 1, lane: *lane }))
                }
                None => {
                    let row0 = &mut rows[0];
                    let idx = row0.echoes.iter().position(|e| *e == id).unwrap_or_else(|| {
                        row0.echoes.push(id);
                        row0.echoes.len() - 1
                    });
                    terms.push((0, coeff, Source::Echo { idx: idx as u16 }));
                }
            }
        }
        out_terms.push(terms);
        out_consts.push(c);
    }

    StripLayout { rows, out_terms, out_consts, k }
}
