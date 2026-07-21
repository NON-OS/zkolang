<!-- NONOS. AGPL-3.0-or-later. -->

# zKølang

A language for verifiable computation, by NØNOS. You write a program; you get back a
transparent post-quantum STARK proof that it ran exactly as written and produced exactly
these outputs, checkable by anyone, forever, with no trusted setup. When you want, the
inputs stay private and the proof reveals only that they satisfied the program.

It is not a general application language. It will not build a web server or a user
interface. It is a proving language, the thing you reach for when you need math you can
prove you did right, and prove it without showing what went in.

## The mental model

A program compiles to a register machine: thirty two registers over the Goldilocks field,
the integers modulo the prime `p = 2^64 - 2^32 + 1`. Running it lays down an execution
trace of width fifty one, and that trace is what the STARK proves. So the whole story is
`program -> trace -> proof`. Every value is a field element, arithmetic wraps modulo `p`,
and the proof binds a fixed statement the verifier replays: the program commitment, the
trace length, and the public inputs and outputs.

## Quickstart

```
cargo build -p nonos_zkolang_cli        # builds the `zkolang` tool
zkolang run examples/cube.zkl --input 9 # compiles, proves, verifies
```

```
verified
outputs [729]
steps 5  trace 2^3
```

The tool has five verbs: `run` compiles and proves, `check` compiles only, `build` emits
a native backend, `key` prints a circuit's registration key, and `fee` prices the
pay-to-prove cost of a run in NOX.

## A first program

```
input x;
let y = x * x * x;
output y;
```

`input` reads a public value, `let` binds an expression, `output` publishes a result.
There are no types to write, because there is one type, a field element. That is also the
one thing to understand: numbers wrap modulo the prime, so `0 - 1` is not minus one, it is
`p - 1`. Comparison and range checks are therefore not free; they are done by witnessed
bit decomposition, which the compiler fills for you.

## The language

- Bindings and values: `let`, `const` (a scalar read by name or a table read by a
  constant index), `input`, `secret`, `output`.
- Control and structure: bounded `for` loops unrolled at compile time, first-class arrays,
  functions inlined at each call, and `include "name.zkl";`.
- Operators: field `+ - * /`, unary `-`, the field inverse `inv`, equality `== !=`,
  ordered comparison `< <= > >=`, logical `! && ||`, the branchless `sel` and its `if`
  and `match` forms.
- A private register: `witness` is `secret`, `public` is `input`, `reveal` is `output`,
  `prove` is `assert`. The same program in a cypherpunk voice:

```
witness key;
public position;
reveal nullifier(key, position);
prove balance == 0;
```

The [specification](SPEC.md) is normative; the [manifesto](MANIFESTO.md) says what it is
for.

## The standard library

Under [stdlib](stdlib), included with `include`: `math` (powers and small gadgets),
`logic` (the boolean gates and a multiplexer and a majority), `cmp` (equality and zero
tests), `order` (min, max, clamp on the comparison range), `field` (reciprocal and
division), `bits` (bit recomposition for range proofs), `poly` (Horner evaluation and
interpolation), `select` (multiplexers over the branchless primitive), `gate` (half and
full adders on bits), `encode` (packing small values into a field element), `curve` (short
Weierstrass point arithmetic), `hash` (the MiMC round, the sixteen-round `permute`, and the
`compress_wide` two-to-one node), `merkle` (`compress2` and the `merkle_step` that climbs an
authentication path), and `vm` (the register machine's one-hot opcode gate, so a program can
verify a run of the language itself). Every gadget is written in zKølang and its soundness is
proven in the suite and, gadget by gadget, in Lean 4 under [lean](lean).

## Backends

One source, four targets, all over the same field. The STARK prover; native **x86_64
assembly** (`zkolang build --target asm`); native **C**; and **Python**. The natives are
checked against the prover bit for bit, and fuzzed against the VM on random programs, so run
and prove are the same verb.

```
zkolang build examples/cube.zkl --target asm --out cube.S
cc cube.S -o cube && ./cube 9        # 729, native, no prover
```

## The compiler

A constant-folding and algebraic-simplification optimizer runs before lowering, so the
trace is smaller while the proof is unchanged. Errors are diagnostics, not kinds: a syntax
error, an undefined name, or an out-of-range index is reported with its line, its column, and
a caret under the offending place.

```
error: unknown variable `foo`
  --> 2:9
   |
   | let y = foo + x;
   |         ^
```

## The utilities

The circuits the kernel and NOX rest on live under [circuits](circuits), each proven with
an accepting and a failing case and pinned to the verifier key an on-chain registry gates
on.

- `circuits/shield` is the private-value utility: `spend_note` proves a note's membership,
  retires it with a nullifier, and range-proves its value; `transfer_note` spends one note
  and creates another, conserving value. The amounts, keys, and positions stay private.
- `circuits/kernel` is the trust boundary: attestation, anti-rollback against a TPM floor,
  capability and syscall authorization, measured boot, and sealing.

A circuit becomes real by registration. Its verifier key is
`keccak256(0x01 ‖ commit ‖ log2N ‖ trace_width ‖ rate ‖ periodic_root)`, its commitment is
`blake3` over the serialized ops, and a pay-to-prove fee settles per use in NOX. Read any
key with `zkolang key <file.zkl>`.

## What it can express

Beyond the utilities, the [examples](examples) reach for real cryptography and for the
language itself. The curve programs prove elliptic curve arithmetic over the field:
`point_add` and `point_double` are the short Weierstrass group law, and `scalar_mul` climbs
the double-and-add ladder to `5*P`, the arithmetic under an elliptic curve signature. The vm
programs turn the language on itself: `verify_run` checks an execution trace and
`verify_registers` checks a register machine run, each using the same one-hot opcode gate the
step AIR uses, so a zKølang proof attests a zKølang execution. Every one is proven end to end,
and each rejects a forged input.

## Recursion

The step AIR is exposed as a standalone `AirExt`, with the transition written once over any
field (`transition_over`) and a `GenericTransition` seam, so a recursive verifier can
arithmetize it as its inner statement and prove that a zKølang proof itself verifies. The
generic composition check lives in the vendored STARK primitives; the registration key a
recursion targets is reproducible from `verifier_key(program, 3)`.

## Proving and verification

```
cargo test -p nonos_zkolang_proofs
```

The suite proves the language end to end: the AIR tamper set and register binding, the
public statement, the optimizer, the operators including comparison, the standard library
gadgets, the shield and kernel utilities and the curve and vm examples with their accept and
reject cases, the verifier-key binding, and the fee model. Nothing here only compiles; it
proves.

It is also fuzzed and machine-checked. The front end must answer any input, however
malformed, with an Ok or an Err and never a panic; the C and assembly backends are checked
against the VM on random programs; and the optimizer is required to preserve every output.
The standard library's soundness is proven in Lean 4 under [lean](lean): the boolean gates
and the byte range, the ordered comparison and the ordering, the encoders and the arithmetic
gates, the powers and the polynomials, the MiMC S-box as a permutation, the opcode gate, and
the field itself with the transfer principle that carries each integer identity into
Goldilocks, with no `sorry` and no axioms.

## Tooling

The `zkolang` command-line tool under [nonos_zkolang_cli](nonos_zkolang_cli), a tree-sitter
grammar under [tree-sitter-zkolang](tree-sitter-zkolang), a TextMate grammar under
[grammars](grammars), and a VS Code extension under [editors/vscode](editors/vscode).

## Layout

```
nonos_zkolang/         the language, the VM, the step AIR, the prover binding
nonos_zkolang_cli/     the zkolang command-line tool
nonos_zkolang_proofs/  the host proof suite
nonos-stark/           the transparent STARK primitives (vendored)
circuits/              the production utilities, kernel and shield
examples/              programs written in the language
stdlib/                the standard library, in zKølang
lean/                  gadget soundness in Lean 4
```

The language and the STARK travel together so the repository builds and proves on its own,
with no dependency outside `blake3`.

## License

AGPL-3.0-or-later. See [LICENSE](LICENSE).
