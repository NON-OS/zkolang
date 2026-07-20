/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The Python field prelude: the modulus and the operations, with inverse by Fermat
//! through Python's built-in modular exponentiation.

pub(super) const PRELUDE: &str = "\
P = 0xFFFFFFFF00000001


def _add(a, b):
    return (a + b) % P


def _sub(a, b):
    return (a - b) % P


def _mul(a, b):
    return (a * b) % P


def _inv(a):
    return pow(a, P - 2, P)
";
