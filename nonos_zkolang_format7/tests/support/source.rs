/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What a boundary of an image reads. */

use nonos_stark::field::Fp;

/** A boundary's source: a value, or a public word. */
#[derive(Debug, PartialEq)]
pub enum Src {
    Value(Fp),
    Word(usize),
}

/** An image's boundaries: column, row, and source. */
pub type Boundaries = Vec<(usize, usize, Src)>;
