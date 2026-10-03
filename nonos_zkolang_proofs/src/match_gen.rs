/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The patterns and values of random `match`es over a value of type `(E, u8)`, where
 * `enum E { A, B(bool), C(u8) }`.
 */

/** A pattern over the scrutinee or one of its parts. */
#[derive(Clone, Debug)]
pub(crate) enum Pat {
    Wild,
    A,
    B(Box<Pat>),
    C(Box<Pat>),
    Bool(bool),
    Range(u8, u8),
    Pair(Box<Pat>, Box<Pat>),
    Or(Box<Pat>, Box<Pat>),
}

/** A value of `E`. */
#[derive(Clone, Copy, Debug)]
pub(crate) enum Ev {
    A,
    B(bool),
    C(u8),
}

/** A value of `E`, a `bool` or a `u8`, or a pair of an `E` and a `u8`. */
#[derive(Clone, Copy, Debug)]
pub(crate) enum Val {
    E(Ev),
    Bool(bool),
    Int(u8),
    Pair(Ev, u8),
}
