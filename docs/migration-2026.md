<!-- NONOS. AGPL-3.0-or-later. -->

# Migrating a program from edition 2025 to edition 2026

This guide maps every form of edition 2025 ([`docs/edition-2025.md`](edition-2025.md)) to
its form in edition 2026 ([`SPEC.md`](../SPEC.md)). A package moves to edition 2026 by
setting `edition = "2026"` in its manifest; a single file is compiled as edition 2026 with
`zkolang check --edition 2026` and the other commands' `--edition` switch (section 20.1).

Each example is a pair: the edition 2025 program, then the edition 2026 program with the
same meaning. The line above a pair names the inputs both are run on, and
`nonos_zkolang_proofs/src/migration_tests.rs` proves the 2025 program with the frozen
compiler, runs the 2026 program, and requires the same outputs for every pair here.

Two forms change meaning (section 20.3): `assert e` asserts that a `bool` is true, where
2025 asserted that a field element is zero; and ordered comparison is defined on typed
integers of any width, where 2025 range-checked field operands to sixteen bits. Every
other change is one of form, or a check that 2026 makes and 2025 did not: types, and the
flow of secret values into public results (section 13).

## 1. The shape of a program

A 2025 program is a list of statements. Its `input` and `secret` declarations, in order,
read the public and secret inputs, and each `output` statement adds a public output. In
2026 the program is `fn main`: each parameter is an input, labelled `public` or `secret`,
and the result is the output.

<!-- run public=3,15 secret=5 -->
```zkolang-2025
input a;
input b;
secret w;
assert a * w == b;
output a + b;
```
```zkolang-2026
fn main(a: public field, b: public field, w: secret field) -> field {
    assert a * w == b;
    a + b
}
```

Several `output` statements become a tuple result, whose parts are the outputs in order.

<!-- run public=7 -->
```zkolang-2025
input a;
output a;
output a * a;
```
```zkolang-2026
fn main(a: public field) -> (field, field) {
    (a, a * a)
}
```

The second spellings of 2025, `public`, `witness`, `prove` and `reveal`, are the same
declarations and statements, and map the same way.

<!-- run public=49 secret=7 -->
```zkolang-2025
public x;
witness w;
prove x - w * w;
reveal x + 1;
```
```zkolang-2026
fn main(x: public field, w: secret field) -> field {
    assert x == w * w;
    x + 1
}
```

## 2. Assertions

A 2025 `assert e;` holds when the field element `e` is zero. In 2026 the condition is a
`bool`, so `assert e;` becomes `assert e == 0;`. The equality forms `assert a == b;` and
`assert a != b;` are written the same in both editions.

<!-- run public=3,8 -->
```zkolang-2025
input x;
input y;
assert x - 3;
assert x != y;
output x + y;
```
```zkolang-2026
fn main(x: public field, y: public field) -> field {
    assert x - 3 == 0;
    assert x != y;
    x + y
}
```

## 3. Secret values in the result

2025 lets any value reach an `output`. In 2026 a value computed from a secret input is
secret, and one that reaches `main`'s public result is an error (E0600) unless it passes
through `declassify`, which marks the place where revealing it is intended (section 13.2).
`zkolang check --declassify` lists every such place.

<!-- run secret=21 -->
```zkolang-2025
secret w;
output w * 2;
```
```zkolang-2026
fn main(w: secret field) -> field {
    declassify(w * 2)
}
```

## 4. Constants

A 2025 constant takes its kind from its initializer: a number, or a table read at a
constant index. A 2026 constant states its type.

<!-- run -->
```zkolang-2025
const K = 7;
const T = [2, 3, 5];
output K * T[1];
```
```zkolang-2026
const K: field = 7;
const T: [field; 3] = [2, 3, 5];

fn main() -> field {
    K * T[1]
}
```

## 5. Functions and blocks

A 2025 function takes field elements and is an expression or a block. A 2026 function
states its parameter and result types; both editions inline every call.

A 2025 block ends in `return e;` or `e`, and either is the block's value. In 2026 a block's
value is its last expression, and `return` leaves the whole function: the 2025 `return` of
a block that is not a function's body is written as the block's last expression.

<!-- run public=3 -->
```zkolang-2025
fn sq(x) = x * x;
fn f(a, b) {
    let s = a + b;
    return sq(s);
}
input x;
let y = {
    let t = x * x;
    return t + 1;
};
output f(y, 1);
```
```zkolang-2026
fn sq(x: field) -> field {
    x * x
}

fn f(a: field, b: field) -> field {
    let s = a + b;
    sq(s)
}

fn main(x: public field) -> field {
    let y = {
        let t = x * x;
        t + 1
    };
    f(y, 1)
}
```

## 6. Loops and values they change

A 2025 loop body rebinds a name with `let`, and the last binding is the name's value after
the loop. In 2026 the variable is declared `let mut` and assigned. The loop variable is a
`usize`, which `as field` converts.

<!-- run public=4 -->
```zkolang-2025
const W = [1, 2, 3];
input x;
let acc = 0;
let s = 0;
for i in 0..5 {
    let acc = acc + i;
}
for i in 0..3 {
    let s = s + W[i] * x;
}
output acc + s;
```
```zkolang-2026
const W: [field; 3] = [1, 2, 3];

fn main(x: public field) -> field {
    let mut acc: field = 0;
    let mut s: field = 0;
    for i in 0..5 {
        acc = acc + i as field;
    }
    for i in 0..3 {
        s = s + W[i] * x;
    }
    acc + s
}
```

## 7. Bits and booleans

In 2025 a comparison is a field element, one or zero, and `&&`, `||`, `!`, `sel` and `if`
work on such bits. In 2026 a comparison is a `bool`; `if c { a } else { b }` replaces
`sel(c, a, b)`; and `b as field` gives a `bool`'s bit where arithmetic needs it. A `bool`
input or result takes one slot holding zero or one, as a 2025 bit does.

<!-- run public=4,4 -->
```zkolang-2025
input a;
input b;
let e = a == b;
output sel(e, 10, 20) + e;
output !(a == 5 || a == 6);
```
```zkolang-2026
fn main(a: public field, b: public field) -> (field, bool) {
    let e = a == b;
    ((if e { 10 } else { 20 }) + e as field, !(a == 5 || a == 6))
}
```

A 2025 input used as a bit becomes a `bool` input, which the program checks is zero or one
as it reads it (section 12.3).

<!-- run public=1,9 -->
```zkolang-2025
input c;
input x;
output if c { x } else { 0 };
```
```zkolang-2026
fn main(c: public bool, x: public field) -> field {
    if c { x } else { 0 }
}
```

## 8. Inverses, division and `match`

`inv(x)` is the method `x.inv()`; `/` on field elements is written the same. A `match` on
a field element with literal arms and a final `_` is written the same.

<!-- run public=3,1 -->
```zkolang-2025
input x;
input op;
output inv(x) * 6 + 12 / x;
output match op { 0 => 10, 1 => 20, _ => 30 };
```
```zkolang-2026
fn main(x: public field, op: public field) -> (field, field) {
    let q = x.inv() * 6 + 12 / x;
    (q, match op { 0 => 10, 1 => 20, _ => 30 })
}
```

## 9. Ordered comparison

2025 compares field elements it range-checks to sixteen bits. 2026 compares integers of
the type they have, and an input's type is checked as the input is read, so a 2025
comparison of inputs becomes one of `u16` inputs. A wider type, such as `u32` or `u64`,
compares wider values, which 2025 could not.

<!-- run public=300,5000 -->
```zkolang-2025
input a;
input b;
output a < b;
output a >= b;
```
```zkolang-2026
fn main(a: public u16, b: public u16) -> (bool, bool) {
    (a < b, a >= b)
}
```

## 10. Includes and the standard library

2026 has no textual include (section 20.2). A file of one's own becomes a module, declared
with `mod name;` and used through `use`; the 2025 standard library's hash and Merkle
functions are in the crate `std`, which every program can name.

<!-- run public=1,2 -->
```zkolang-2025
include "hash.zkl";
input l;
input r;
output compress_wide(l, r);
```
```zkolang-2026
use std::hash::compress_wide;

fn main(l: public field, r: public field) -> field {
    compress_wide(l, r)
}
```

The other 2025 library files are small formulas over field elements, and each is a line of
2026: `sq(x)` is `x * x`, `mux(s, a, b)` is `if s { a } else { b }` for a `bool` `s`, and
`min(a, b)` on integers is `a.min(b)`. `std::cmp`, `std::poly` and `std::curve` hold the
ones that are more than a line.

## 11. Tuples and destructuring

Tuples, `let (a, b) = ...;` and `_` in a binding are written the same.

<!-- run public=6 -->
```zkolang-2025
input x;
let (q, _) = (x, x + 1);
output q * 2;
```
```zkolang-2026
fn main(x: public field) -> field {
    let (q, _) = (x, x + 1);
    q * 2
}
```
