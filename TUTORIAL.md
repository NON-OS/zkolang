<!-- NONOS. AGPL-3.0-or-later. -->

# Getting started with zKølang

A hands-on introduction. By the end you will have written a program, proven it, kept an
input private, and priced the proof. Every command here runs; every output is real.

## Install the tool

From the repository root:

```
cargo build -p nonos_zkolang_cli
```

That builds `zkolang`, the command-line tool. The examples below invoke it from the repo
root; put it on your path if you like.

## Your first program

Create `cube.zkl`:

```
input x;
let y = x * x * x;
output y;
```

Run it:

```
zkolang run cube.zkl --input 9
```

```
verified
outputs [729]
steps 5  trace 2^3
```

`input` reads a public value, `let` binds an expression, `output` publishes a result. The
word that matters is `verified`: the tool did not just compute the cube, it produced a
proof that this exact program on this input produced this output, and checked it.

## The one thing to understand: the field

Every value is an element of the Goldilocks field, the integers modulo the prime
`p = 2^64 - 2^32 + 1`. Arithmetic wraps around that prime. This is not sixty four bit
integers. See it:

```
echo 'output 0 - 1;' > wrap.zkl
zkolang run wrap.zkl
```

Zero minus one is not minus one, it is `p - 1`, a huge number. There are no negatives,
only residues. This is why comparison and range checks are not free: to decide an order
you cannot just subtract, because subtraction wraps. The language does the bit
decomposition for you, but it costs more than an add.

## Public and private

An input can be public, bound into the proof for everyone to see, or a private witness,
hidden. Prove someone is old enough without revealing their birth year:

```
public this_year;
public min_age;
witness birth_year;

prove ((birth_year + min_age) <= this_year) - 1;
```

`witness` is a private input, `public` is a public one, `prove` asserts a constraint. The
words `public`, `witness`, `reveal`, and `prove` are the same language as `input`,
`secret`, `output`, and `assert`; use either.

```
zkolang run age.zkl --input 2026,18 --witness 2000
```

Verified: born in 2000, at least eighteen, and the verifier never learned the year. Try a
witness of 2015, someone too young, and there is no proof at all. A false statement in
this language is not something you can prove.

## Building up

The language has the shapes you expect. Bindings, functions, arrays, and bounded loops:

```
fn sq(x) = x * x;
const W = [3, 1, 4, 1, 5];
input x;
let acc = 0;
for i in 0 .. 5 {
    let acc = acc + W[i] * sq(x);
}
output acc;
```

A function does not have to be one expression. Open a block and it has a body: name the
intermediate steps, then return the result.

```
fn quadratic(x, a, b, c) {
    let sq = x * x;
    let linear = b * x;
    return a * sq + linear + c;
}
input x;
output quadratic(x, 2, 3, 1);
```

The bindings scope to the block and to each other in order; the last statement is the
value, written `return e;` or as a trailing expression with no `return`. A block is an
expression like any other, so it can stand anywhere a value can, not only as a body.

A function can return more than one value. A compare-swap, the step every sorting network
is built from, returns the smaller and the larger of a pair, and the caller destructures
the two with `let (a, b)`.

```
fn minmax(a, b) {
    let ordered = a < b;
    return (sel(ordered, a, b), sel(ordered, b, a));
}
input x;
input y;
let (lo, hi) = minmax(x, y);
output lo;
output hi;
```

The name count must match how many values the function returns, or the compiler says so.
Use `_` for a value you do not need: `let (_, hi) = minmax(x, y);` keeps only the larger.
Destructuring works inside a function body too, so one function can build on another's two
results, which is how a median or a small sort reads.

A function can take a whole array as an argument and index it inside, so an operation over
a vector is named once instead of spelled out at every call.

```
fn dot3(a, b) = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
input p; input q; input r;
input s; input t; input u;
let v = [p, q, r];
let w = [s, t, u];
output dot3(v, w);
```

A function can also return an array, so an elementwise operation builds a vector the caller
binds and indexes. To feed one function's vector into another, bind it first.

```
fn scale3(v, k) = [v[0] * k, v[1] * k, v[2] * k];
input a; input b; input c;
let u = [a, b, c];
let s = scale3(u, 3);
output s[0]; output s[1]; output s[2];
```

Functions inline at each call, loops unroll at compile time, arrays are indexed by a
constant. There is also ordered comparison (`< <= > >=`), boolean logic (`! && ||`), a
branchless `sel`, and a `match`:

```
public op;
public a;
public b;
reveal match op {
    0 => a + b,
    1 => a - b,
    _ => a * b,
};
```

## Check errors, before proving

`check` compiles without proving, and it tells you where you are wrong:

```
$ zkolang check broken.zkl
error: unknown variable `foo`
  --> 2:10
```

## A real thing: a range proof

Prove a value is a byte, in `[0, 256)`, by exhibiting its bits as a private witness:

```
include "bits.zkl";
input v;
secret b0; secret b1; secret b2; secret b3;
secret b4; secret b5; secret b6; secret b7;
assert bit(b0); assert bit(b1); assert bit(b2); assert bit(b3);
assert bit(b4); assert bit(b5); assert bit(b6); assert bit(b7);
assert v - compose8(b0, b1, b2, b3, b4, b5, b6, b7);
```

An out-of-range value has no bit witness, so it has no proof. This is the gadget that
keeps a shielded amount non-negative. The `circuits/` folder builds it into a full
confidential transfer.

## Emit, price, register

The same program compiles to native code:

```
zkolang build cube.zkl --target c --out cube.c
cc cube.c -o cube && ./cube 9        # 729, no prover
```

It has a price to prove, in NOX:

```
zkolang fee cube.zkl --input 9
```

And a registration identity, the verifier key an on-chain registry gates on:

```
zkolang key cube.zkl
```

## Where to go next

- The [specification](SPEC.md) is the normative reference.
- The [standard library](stdlib) is small and readable; every gadget is one expression.
- The [circuits](circuits) are the real utilities: a shielded spend and transfer, and the
  kernel's attestation and anti-rollback.
- The [examples](examples) reach further: elliptic curve point addition, doubling, and
  scalar multiplication, and a circuit that verifies a run of the language itself.
- The [manifesto](MANIFESTO.md) says what the language is for.

Write a `.zkl`, prove it, and you have a checkable statement no one has to trust you for.
