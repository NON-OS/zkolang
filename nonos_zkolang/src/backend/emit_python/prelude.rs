/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The Python field prelude: the modulus and the operations, with inverse by Fermat
 * through Python's built-in modular exponentiation. A constraint the proof enforces
 * raises `Unprovable` when it fails, in code `python -O` keeps, unlike `assert`; an
 * input that is not a field element raises `ValueError`.
 */

pub(super) const PRELUDE: &str = "\
P = 0xFFFFFFFF00000001


def _add(a, b):
    return (a + b) % P


def _sub(a, b):
    return (a - b) % P


def _mul(a, b):
    return (a * b) % P


class Unprovable(Exception):
    \"\"\"A constraint the proof enforces failed, so no proof of this run exists.\"\"\"


def _check(ok, what):
    if not ok:
        raise Unprovable(what)


def _inv(a):
    _check(a != 0, \"inverse of zero\")
    return pow(a, P - 2, P)


def _sel(c, a, b):
    _check(c in (0, 1), \"select on a condition that is not a bit\")
    return a if c else b


def _input(i, v):
    if type(v) is not int or not 0 <= v < P:
        raise ValueError(f\"input {i} is not an integer below the field modulus\")
    return v
";
