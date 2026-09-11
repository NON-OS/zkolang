// NONOS Operating System (AGPL-3.0-or-later)
//! The recording felt. An inner AIR's transition code runs over `Ext2<Cell>`
//! and every field operation lands on a tape as a node; the code itself is
//! never parsed or changed. The tape is the arithmetic DAG the strip compiler
//! lays onto rows. Recording is host-side tooling, never prover-path code.

use crate::crypto::stark::field::{Felt, Fp};
use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Node {
    Input(u32),
    Const(Fp),
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
}

thread_local! {
    static TAPE: RefCell<Vec<Node>> = const { RefCell::new(Vec::new()) };
}

/// One recorded value: an index into the thread's tape. `Copy` so the
/// constraint code moves it like the field element it stands in for.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell(pub u32);

fn push(n: Node) -> Cell {
    TAPE.with(|t| {
        let mut t = t.borrow_mut();
        t.push(n);
        Cell(t.len() as u32 - 1)
    })
}

/// Reset the tape and seed it: ZERO, ONE, then `n_inputs` input nodes, whose
/// cells are returned in order.
pub fn begin(n_inputs: usize) -> Vec<Cell> {
    TAPE.with(|t| {
        let mut t = t.borrow_mut();
        t.clear();
        t.push(Node::Const(Fp::ZERO));
        t.push(Node::Const(Fp::ONE));
        (0..n_inputs)
            .map(|i| {
                t.push(Node::Input(i as u32));
                Cell(t.len() as u32 - 1)
            })
            .collect()
    })
}

/// The recorded tape, cloned out.
pub fn snapshot() -> Vec<Node> {
    TAPE.with(|t| t.borrow().clone())
}

impl Felt for Cell {
    const ZERO: Cell = Cell(0);
    const ONE: Cell = Cell(1);

    fn from_base(x: Fp) -> Cell {
        push(Node::Const(x))
    }

    fn pow(self, exp: u64) -> Cell {
        // Square-and-multiply as recorded muls, so the strip sees products
        // it can witness instead of an opaque power.
        let mut acc = Cell::ONE;
        let mut base = self;
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                acc = acc * base;
            }
            e >>= 1;
            if e > 0 {
                base = base * base;
            }
        }
        acc
    }

    fn inv(self) -> Cell {
        panic!("the compose strip requires polynomial transitions; an inner called inv");
    }
}

impl core::ops::Add for Cell {
    type Output = Cell;
    fn add(self, rhs: Cell) -> Cell {
        push(Node::Add(self.0, rhs.0))
    }
}

impl core::ops::Sub for Cell {
    type Output = Cell;
    fn sub(self, rhs: Cell) -> Cell {
        push(Node::Sub(self.0, rhs.0))
    }
}

impl core::ops::Mul for Cell {
    type Output = Cell;
    fn mul(self, rhs: Cell) -> Cell {
        push(Node::Mul(self.0, rhs.0))
    }
}
